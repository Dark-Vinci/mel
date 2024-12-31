
#[tokio::main]
async fn main() {
    async {
        println!("Hello, world!");
    }.await;
    println!("Hello, world!");
}
