mod modules;
mod app;
mod config;

use app::App;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::default();
    let app_result = app.run(terminal).await;
    ratatui::restore();
    app_result
}
