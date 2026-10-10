fn main() {
    console_error_panic_hook::set_once();
    let game = racer_web::game();
    yew::Renderer::<microscope::page::GamePage>::with_props(game).render();
}
