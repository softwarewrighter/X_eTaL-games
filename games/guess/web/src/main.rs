fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<guess_web::app::App>::new().render();
}
