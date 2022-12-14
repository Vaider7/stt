use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct MicProps {
    pub class: AttrValue,
}

#[function_component]
pub fn Mic(props: &MicProps) -> Html {
    html! {
            <svg class={props.class.to_string()} version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" x="0px" y="0px"
         viewBox="0 0 300 300" style="enable-background:new 0 0 300 300;">
    <g>
        <g>
            <g>
                <path d="M150,222.581c32.018,0,58.065-26.047,58.065-58.065V58.065C208.065,26.047,182.018,0,150,0S91.935,26.047,91.935,58.065
				v106.452C91.935,196.534,117.982,222.581,150,222.581z M101.615,96.775c2.671,0,4.839-2.168,4.839-4.839
				c0-2.671-2.168-4.839-4.839-4.839V62.903c2.671,0,4.839-2.168,4.839-4.839c0-2.584-2.042-4.655-4.597-4.79
				c1.706-17.293,12.583-31.897,27.666-38.985c0.368,0.092,0.726,0.227,1.123,0.227c1.723,0,3.165-0.953,4.021-2.308
				c3.373-1.132,6.89-1.926,10.544-2.289c0.135,2.555,2.206,4.597,4.79,4.597c2.584,0,4.655-2.042,4.79-4.597
				c3.653,0.363,7.171,1.156,10.544,2.289c0.856,1.355,2.298,2.308,4.021,2.308c0.397,0,0.755-0.135,1.123-0.227
				c15.082,7.089,25.955,21.692,27.663,38.985c-2.55,0.135-4.592,2.206-4.592,4.79c0,2.671,2.168,4.839,4.839,4.839v24.194
				c-2.671,0-4.839,2.168-4.839,4.839c0,2.671,2.168,4.839,4.839,4.839v9.677h-96.774V96.775z M101.613,116.129h96.774v9.677
				h-96.774V116.129z M101.613,135.484h96.774v29.032c0,26.681-21.706,48.387-48.387,48.387c-26.681,0-48.387-21.706-48.387-48.387
				V135.484z"/>
                <path d="M217.744,164.516h-0.002c0,37.355-30.387,67.742-67.742,67.742c-37.355,0-67.742-30.387-67.742-67.742v-62.903h-9.677
				v62.903c0,35.995,24.731,66.242,58.065,74.869v50.937H72.581V300h58.065h38.71h58.065v-9.677h-58.065v-50.937
				c33.334-8.627,58.065-38.874,58.065-74.869v-62.903h-9.677V164.516z M140.322,290.323v-49.06
				c3.179,0.402,6.392,0.673,9.677,0.673c3.285,0,6.498-0.271,9.677-0.673v49.06H140.322z"/>
                <rect x="53.226" y="101.613" width="9.677" height="33.871"/>
                <rect x="237.097" y="101.613" width="9.677" height="33.871"/>
                <circle cx="120.968" cy="91.935" r="4.839"/>
                <circle cx="140.323" cy="91.935" r="4.839"/>
                <circle cx="159.677" cy="91.935" r="4.839"/>
                <circle cx="179.032" cy="91.935" r="4.839"/>
                <circle cx="111.29" cy="72.581" r="4.839"/>
                <circle cx="130.645" cy="72.581" r="4.839"/>
                <circle cx="150" cy="72.581" r="4.839"/>
                <circle cx="169.355" cy="72.581" r="4.839"/>
                <circle cx="188.71" cy="72.581" r="4.839"/>
                <circle cx="120.968" cy="58.065" r="4.839"/>
                <circle cx="140.323" cy="58.065" r="4.839"/>
                <circle cx="159.677" cy="58.065" r="4.839"/>
                <circle cx="179.032" cy="58.065" r="4.839"/>
                <circle cx="111.29" cy="43.548" r="4.839"/>
                <circle cx="130.645" cy="43.548" r="4.839"/>
                <circle cx="150" cy="43.548" r="4.839"/>
                <circle cx="169.355" cy="43.548" r="4.839"/>
                <circle cx="188.71" cy="43.548" r="4.839"/>
                <circle cx="120.968" cy="29.032" r="4.839"/>
                <circle cx="140.323" cy="29.032" r="4.839"/>
                <circle cx="159.677" cy="29.032" r="4.839"/>
                <circle cx="179.032" cy="29.032" r="4.839"/>
            </g>
        </g>
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
