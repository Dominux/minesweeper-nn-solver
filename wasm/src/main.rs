mod components;
mod models;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<components::settings::Settings>();
}
