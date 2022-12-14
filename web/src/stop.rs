use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct StopProps {
    #[prop_or_default]
    pub class: AttrValue,
}

#[function_component]
pub fn Stop(props: &StopProps) -> Html {
    html! {
            <svg class={props.class.to_string()} version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px"
         viewBox="0 0 10.334 10.334" style="enable-background:new 0 0 10.334 10.334;">
    <g>
        <path d="M10.333,9.816c0,0.285-0.231,0.518-0.517,0.518H0.517C0.233,10.334,0,10.102,0,9.816V0.517
            C0,0.232,0.231,0,0.517,0h9.299c0.285,0,0.517,0.231,0.517,0.517V9.816z"/>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    <g>
    </g>
    </svg>

        }
}
