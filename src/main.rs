use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    println!("http://127.0.0.1:{}", port);

    run(listener)?.await
}
