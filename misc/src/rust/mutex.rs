use owning_ref::MutexGuardRef;
use std::sync::Mutex;
use std::sync::{Arc, RwLock};

use std::thread;
use std::time::Duration;

type SharedList = Arc<RwLock<Vec<i32>>>;

/// <https://tokio.rs/tokio/tutorial/shared-state>
///

pub struct CanIncrement {
    mutex: Mutex<i32>,
}
#[derive(Clone)]
pub struct ShareMe {
    list: SharedList,
}

impl ShareMe {
    pub fn new(list: Vec<i32>) -> ShareMe {
        ShareMe {
            list: Arc::new(RwLock::new(list)),
        }
    }

    pub fn push(&self, item: i32) {
        let mut list = self.list.write().unwrap();
        list.push(item);
    }

    pub fn len(&self) -> usize {
        let list = self.list.read().unwrap();
        list.len()
    }
}

impl CanIncrement {
    pub fn new(increment: i32) -> CanIncrement {
        let can = CanIncrement {
            mutex: Mutex::new(increment),
        };
        can
    }

    pub fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }

    /// <https://stackoverflow.com/questions/40095383/how-to-return-a-reference-to-a-sub-value-of-a-value-that-is-under-a-mutex>
    /// <http://kimundi.github.io/owning-ref-rs/owning_ref/index.html>
    pub fn get(&self) -> MutexGuardRef<i32> {
        MutexGuardRef::new(self.mutex.lock().unwrap())
    }
}

async fn increment_and_do_stuff(can_increment: &CanIncrement) {
    can_increment.increment();
    do_something_async();
}

async fn do_something_async() {
    println!("**** Doing something async *****");
}

fn thread_function(list: ShareMe) {
    loop {
        println!("::::: {} ::::::", list.len());
        thread::sleep(Duration::from_millis(100));
    }
}

/// from the docs...
///
/// <https://play.rust-lang.org/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Ause%20std%3A%3Async%3A%3A%7BArc%2C%20Mutex%7D%3B%0Ause%20std%3A%3Athread%3B%0Ause%20std%3A%3Async%3A%3Ampsc%3A%3Achannel%3B%0A%0Aconst%20N%3A%20usize%20%3D%2010%3B%0A%0A%2F%2F%20Spawn%20a%20few%20threads%20to%20increment%20a%20shared%20variable%20(non-atomically)%2C%20and%0A%2F%2F%20let%20the%20main%20thread%20know%20once%20all%20increments%20are%20done.%0A%2F%2F%0A%2F%2F%20Here%20we%27re%20using%20an%20Arc%20to%20share%20memory%20among%20threads%2C%20and%20the%20data%20inside%0A%2F%2F%20the%20Arc%20is%20protected%20with%20a%20mutex.%0Alet%20data%20%3D%20Arc%3A%3Anew(Mutex%3A%3Anew(0))%3B%0A%0Alet%20(tx%2C%20rx)%20%3D%20channel()%3B%0Afor%20_%20in%200..N%20%7B%0A%20%20%20%20let%20(data%2C%20tx)%20%3D%20(Arc%3A%3Aclone(%26data)%2C%20tx.clone())%3B%0A%20%20%20%20thread%3A%3Aspawn(move%20%7C%7C%20%7B%0A%20%20%20%20%20%20%20%20%2F%2F%20The%20shared%20state%20can%20only%20be%20accessed%20once%20the%20lock%20is%20held.%0A%20%20%20%20%20%20%20%20%2F%2F%20Our%20non-atomic%20increment%20is%20safe%20because%20we%27re%20the%20only%20thread%0A%20%20%20%20%20%20%20%20%2F%2F%20which%20can%20access%20the%20shared%20state%20when%20the%20lock%20is%20held.%0A%20%20%20%20%20%20%20%20%2F%2F%0A%20%20%20%20%20%20%20%20%2F%2F%20We%20unwrap()%20the%20return%20value%20to%20assert%20that%20we%20are%20not%20expecting%0A%20%20%20%20%20%20%20%20%2F%2F%20threads%20to%20ever%20fail%20while%20holding%20the%20lock.%0A%20%20%20%20%20%20%20%20let%20mut%20data%20%3D%20data.lock().unwrap()%3B%0A%20%20%20%20%20%20%20%20*data%20%2B%3D%201%3B%0A%20%20%20%20%20%20%20%20if%20*data%20%3D%3D%20N%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20tx.send(()).unwrap()%3B%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20%2F%2F%20the%20lock%20is%20unlocked%20here%20when%20%60data%60%20goes%20out%20of%20scope.%0A%20%20%20%20%7D)%3B%0A%7D%0A%0Arx.recv().unwrap()%3B%0A%7D&edition=2018>
/// <https://doc.rust-lang.org/std/sync/struct.Mutex.html>
///

#[cfg(test)]
mod tests {
    use super::*;

    use tokio::net::TcpListener;
    use tokio::prelude::*;

    /// <https://blog.x5ff.xyz/blog/async-tests-tokio-rust/>
    #[actix_rt::test]
    async fn test_increment_and_do_stuff() {
        let ic = CanIncrement::new(0);

        ic.increment();
        ic.increment();

        increment_and_do_stuff(&ic).await;

        let increments = ic.get();

        assert_eq!(*increments, 3);
    }

    /// <https://stackoverflow.com/questions/43592247/how-to-tell-rust-to-let-me-modify-a-shared-variable-hidden-behind-an-rwlock>
    #[test]
    #[ignore] // it is too expensive
    fn test_share_me() {
        println!(":::: share me? ::::::::::::");
        let v = vec![0, 1, 2, 3, 4, 5, 6];
        let share_me = ShareMe::new(v);

        for _ in 0..10 {
            let v = share_me.clone();
            thread::spawn(move || thread_function(v));
        }

        for t in 0..5 {
            share_me.push(t);
            thread::sleep(Duration::from_millis(1000));
        }
    }

    #[tokio::test]
    async fn test_testing() -> Result<(), Box<dyn std::error::Error>> {
        let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

        Ok(())
    }
}
