use newsletter::configuration::get_configuration;
use newsletter::startup::Application;
use newsletter::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration file");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
