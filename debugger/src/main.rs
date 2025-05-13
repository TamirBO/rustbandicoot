mod app;
mod emulator;
mod ui;


use app::App;

const TITLE: &str = "PlayStation Emulator";
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    app.run_app(TITLE);
}
