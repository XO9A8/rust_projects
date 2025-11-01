use tokio::time::{self, Duration};
#[allow(unused_variables, dead_code)]
async fn say_hello() {
    println!("Hello");
    time::sleep(Duration::from_millis(500)).await;
    println!("...from say_hello!");
}
#[allow(unused_variables, dead_code)]
async fn say_world() {
    println!("World");
    time::sleep(Duration::from_millis(250)).await;
    println!("...from say_world!");
}
async fn fetch_data() -> Result<String, &'static str> {
    time::sleep(Duration::from_secs(1)).await;
    Err("Data fetched.")
}
#[allow(unused_variables, dead_code)]
#[tokio::main]
async fn main() {
    let (str1, str2) = tokio::join!(fetch_data(), fetch_data());
    match (str1, str2) {
        (Ok(n), Ok(y)) => {
            //printign code
        }
        _ => (),
    }
}
