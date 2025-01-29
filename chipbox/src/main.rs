mod app;
pub mod hot;

fn main() -> miette::Result<()> {
    let mut app = app::App::new();
    app.run()
}
