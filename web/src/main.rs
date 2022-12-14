extern crate core;

mod app;
mod mic;
mod stop;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
