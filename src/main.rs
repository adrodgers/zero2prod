// use crate::startup::run;
// use std::net::TcpListener;
// use zero2prod::startup;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let address = TcpListener::bind("127.0.0.1:8000")?;
//     run(address)?.await
// }
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
