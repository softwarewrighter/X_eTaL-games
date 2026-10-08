fn main() {
    console_error_panic_hook::set_once();
    let game = lights_out_web::game();
    yew::Renderer::<microscope::page::GamePage>::with_props(game).render();
}
