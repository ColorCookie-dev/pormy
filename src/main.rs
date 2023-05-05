mod stopwatch;
mod terminal;
mod termion_term;
mod stopwatch_app;

fn main() -> anyhow::Result<()> {
    // log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    
    let mut app = 
        stopwatch_app::StopwatchApp::<termion_term::TermionScreen>::new()?;

    while app.to_quit() != true {
        app.update()?;
    }

    Ok(())
}
