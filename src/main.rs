use delibird::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new().map_err(|err| {
        ratatui::restore();
        color_eyre::eyre::eyre!(err)
    })?;

    let result = app.run(terminal);
    ratatui::restore();
    result
}
