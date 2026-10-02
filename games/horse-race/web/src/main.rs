fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<horse_race_web::app::App>::new().render();
}
