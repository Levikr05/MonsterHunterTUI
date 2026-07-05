mod app;
mod ui;

use app::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.run()
}