pub fn make_canvas(context: &sdl2::Sdl, width: u32, height: u32) -> sdl2::render::Canvas<sdl2::video::Window> {
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .unwrap();

    window.into_canvas().build().unwrap()
}