use crate::startup::run;
use std::net::TcpListener;
use zero2prod::startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
