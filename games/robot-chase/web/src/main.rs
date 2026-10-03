fn main() {
    console_error_panic_hook::set_once();
    let game = robot_chase_web::game();
    yew::Renderer::<microscope::page::GamePage>::with_props(game).render();
}
