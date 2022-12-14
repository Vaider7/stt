use crate::mic::Mic;
use crate::stop::Stop;
use js_sys::{Int16Array, Promise};
use serde::Deserialize;
use serde_json::json;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, AudioContext, AudioProcessingEvent, MediaStreamConstraints, MessageEvent, ScriptProcessorNode, Worker, MediaStream, MediaStreamAudioSourceNode};
use yew::html::Scope;
use yew::prelude::*;

pub fn get_micro() -> Promise {
    let mut media_stream_constraints = MediaStreamConstraints::new();
    let opts = media_stream_constraints.audio(&JsValue::TRUE);

    window()
        .unwrap()
        .navigator()
        .media_devices()
        .unwrap()
        .get_user_media_with_constraints(opts)
        .unwrap()
}

pub fn create_audio_processor(
    audio_ctx: &AudioContext,
    sample_rate: f32,
    scope: Scope<App>,
) -> ScriptProcessorNode {
    let processor = audio_ctx.create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(4096, 1, 1).unwrap();

    let downsampler =
        Worker::new("/assets/downsampling_worker.js").expect("Adding worker");

    let init_message = JsValue::from(format!(
        "{{\"command\": \"init\", \"inputSampleRate\": {}}}",
        sample_rate
    ));

    downsampler
        .post_message(&init_message)
        .expect("Sending init message");

    let process_message = move |event: MessageEvent| {
        let chunk = event.data();

        let chunk = Int16Array::from(chunk).to_vec();

        scope.send_message(AppMsg::AddChunk(chunk));
    };

    let on_message = Closure::wrap(Box::new(process_message) as Box<dyn Fn(MessageEvent)>);
    downsampler.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

    on_message.forget();

    let process_audio = move |event: AudioProcessingEvent| {
        let data = event.input_buffer().unwrap().get_channel_data(0).unwrap();
        downsampler
            .post_message(&JsValue::from(format!(
                "{{\"command\": \"process\", \"inputFrame\": {:?}}}",
                data
            )))
            .expect("Sending process message");
    };

    let on_audio_process =
        Closure::wrap(Box::new(process_audio) as Box<dyn Fn(AudioProcessingEvent)>);

    processor.set_onaudioprocess(Some(on_audio_process.as_ref().unchecked_ref()));

    on_audio_process.forget();

    processor
        .connect_with_audio_node(&audio_ctx.destination())
        .expect("Adding destination node");

    processor
}

#[derive(Deserialize)]
pub struct Response {
    text: String,
}

#[derive(Clone)]
pub struct App {
    pub is_recording: bool,
    pub is_micro_ready: bool,
    pub recognition: String,
    pub audio_ctx: AudioContext,
    pub processor: Option<ScriptProcessorNode>,
    pub media_stream: Option<MediaStream>,
    pub media_stream_source: Option<MediaStreamAudioSourceNode>,
    pub chunks: Vec<i16>,
    pub is_recognition: bool,
}

pub enum AppMsg {
    GetMicrophone,
    SetMediaRecorder(JsValue),
    RenderRecognition(String),
    AddChunk(Vec<i16>),
    SendToRecognize,
    ChangeRecordingStatus,
    ChangeRecognitionStatus,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_recording: false,
            is_micro_ready: false,
            is_recognition: false,
            recognition: String::new(),
            audio_ctx: AudioContext::new().unwrap_throw(),
            media_stream: None,
            media_stream_source: None,
            processor: None,
            chunks: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::GetMicrophone => {
                if self.media_stream.is_none() {
                    ctx.link().send_future_batch(async {
                        let value = JsFuture::from(get_micro()).await.unwrap_throw();
                        vec![
                            AppMsg::SetMediaRecorder(value),
                            AppMsg::ChangeRecordingStatus,
                        ]
                    });
                } else if self.is_recording {
                    ctx.link().send_message(AppMsg::SendToRecognize);
                    self.media_stream = None;
                    self.media_stream_source.as_ref().unwrap().disconnect().unwrap();
                    self.media_stream_source = None;
                    self.processor.as_ref().unwrap().disconnect().unwrap();

                } else {
                    self.is_recording = true;
                }
                true
            }

            AppMsg::ChangeRecognitionStatus => {
                self.is_recognition = !self.is_recognition;
                true
            }

            AppMsg::ChangeRecordingStatus => {
                self.is_recording = !self.is_recording;
                true
            }

            AppMsg::SendToRecognize => {
                self.is_recognition = true;
                self.is_recording = false;
                let chunks = self.chunks.clone();

                let scope = ctx.link().clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = gloo_net::http::Request::post("http://127.0.0.1:3000/process")
                        .json(&json!({ "chunks": chunks }))
                        .expect("Creating json")
                        .send()
                        .await
                        .expect("Sending request")
                        .json::<Response>()
                        .await
                        .unwrap();

                    scope.send_message_batch(vec![
                        AppMsg::RenderRecognition(response.text),
                        AppMsg::ChangeRecognitionStatus,
                    ])
                });

                true
            }

            AppMsg::SetMediaRecorder(media_stream) => {
                self.media_stream = Some(MediaStream::from(media_stream));

                self.media_stream_source = Some(
                    self.audio_ctx
                        .create_media_stream_source(self.media_stream.as_ref().unwrap())
                        .unwrap(),
                );
                let sample_rate = self.audio_ctx.sample_rate();
                let scope = ctx.link().to_owned();
                self.processor = Some(create_audio_processor(&self.audio_ctx, sample_rate, scope));
                self.media_stream_source
                    .as_ref()
                    .unwrap()
                    .connect_with_audio_node(self.processor.as_ref().unwrap())
                    .expect("Connecting processor");

                false
            }

            AppMsg::RenderRecognition(val) => {
                self.recognition = val;
                self.chunks.clear();
                true
            }
            AppMsg::AddChunk(chunk) => {
                for i in chunk {
                    self.chunks.push(i);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("background", "is-centered", "is-flex")}>
                <div class={classes!("block", "is-centered", "is-flex")}>
                    if self.is_recording
                        {<Stop class={classes!("stop").to_string()} />}
                    else
                        {<Mic class={classes!("mic").to_string()} />}
                    <div class="circle" />
                    <div class="out-circle" onclick={ctx.link().callback(|_| AppMsg::GetMicrophone)} style={if self.is_recognition {"pointer-events: none;"} else {""}} />
                </div>

                if self.is_recognition
                    {<div class="rectangles">
                        <div class={classes!("rect", "rect1")}></div>
                        <div class={classes!("rect", "rect2")}></div>
                        <div class={classes!("rect", "rect3")}></div>
                    </div>}
                else
                    {<div class="text">
                        if self.is_recording
                            {<div class={"crutch"}></div>}
                        else if self.recognition.is_empty() && !self.is_recording
                            {<p>{"Нажмите на кнопку для начала записи"}</p>}
                        else
                            {<p>{&self.recognition}</p>}
                    </div>}
            </div>
        }
    }
}
