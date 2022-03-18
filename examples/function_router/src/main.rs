mod app;
mod components;
mod content;
mod generator;
mod pages;

pub use app::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}