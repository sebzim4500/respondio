use respondio;
use tokio;

#[respondio::get("/todo")]
async fn get_todo() -> &'static str {
    println!("Got request");
    "Hello world"
}

#[respondio::get("/reverse/{input}")]
async fn reverse(input: String) -> String {
    input.chars().rev().collect()
}

#[respondio::get("/square/{number}")]
async fn square(number: u32) -> String {
    format!("result = {}", number * number)
}

#[respondio::get("/demo/{}/{}/{a}/{b}/{c}/{d}")]
async fn path_var_demo(a: usize, c: String, b: u8) -> String {
    format!("a = {}, b = {}, c = {}", a, b, c)
}

#[tokio::main]
async fn main() {
    respondio::run_server(&"127.0.0.1:8080".parse().unwrap()).await;
}
