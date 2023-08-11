use std::net::TcpListener;

use newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("http://127.0.0.1:0").expect("Could not bind system assigned address");
    
    run(listener)?.await
}
