use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // get the world from the Universe
    let op = say_world();

    println!("hello");

    let world_str = op.await;

    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", world_str.into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}

async fn say_world() -> &'static str {
    println!("world");
    "world"
}
