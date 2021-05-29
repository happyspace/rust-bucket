use bytes::Bytes;
use mini_redis::{Connection, Frame};
use parking_lot::Mutex as ParkingLotMutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

type Dd = Arc<ParkingLotMutex<HashMap<String, Bytes>>>;

/// <https://c9x.me/articles/gthreads/intro.html>
/// <https://users.rust-lang.org/t/green-threads-vs-async/42159>
/// <https://smallcultfollowing.com/babysteps/blog/2020/04/30/async-interviews-my-take-thus-far/>
/// <https://www.infoq.com/presentations/rust-2019/>
/// <https://en.wikipedia.org/wiki/Green_threads>
///
/// See <https://users.rust-lang.org/t/solved-channel-in-a-loop-in-a-thread-borrowed-value-does-not-live-long-enough/26733/4>
/// for placement of the db.clone()

#[tokio::main]
async fn main() {
    println!(":: Redis Server ::");
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db: Dd = Arc::new(ParkingLotMutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        let dbc = db.clone();
        tokio::spawn(async move {
            // Clone the handle to the hash map.

            process(socket, dbc).await;
        });
    }
}

/// <https://tokio.rs/tokio/tutorial/spawning>
///  // The `Connection` lets us read/write redis **frames** instead of
///    // byte streams. The `Connection` type is defined by mini-redis.
/// <https://users.rust-lang.org/t/what-does-self-mean-in-use/15559>
///
/// Full code has been updated from the example ...
/// <https://github.com/tokio-rs/website/blob/master/tutorial-code/spawning/src/main.rs>
///
async fn process(socket: TcpStream, db: Dd) {
    // self adds the mini_redis::Command as well as Get and Set
    use mini_redis::Command::{self, Get, Set};

    // A hashmap is used to store data
    //
    // let mut db = HashMap::new();

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                /// <https://docs.rs/parking_lot/0.11.0/parking_lot/type.Mutex.html>
                // The shared state can only be accessed once the lock is held.
                // Our non-atomic increment is safe because we're the only thread
                // which can access the shared state when the lock is held.
                let mut db = db.lock();
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("unimplemented: {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
