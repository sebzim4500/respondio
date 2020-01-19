use tokio;

#[respondio::get("/todo")]
async fn get_todo() -> &'static str {
    println!("Got request");
    "Hello world"
}

#[tokio::main]
async fn main() {
    respondio::run_server(&"127.0.0.1:8080".parse().unwrap()).await;
}