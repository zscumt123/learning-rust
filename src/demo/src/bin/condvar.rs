use std::sync::{mpsc::sync_channel, Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        eprintln!("I'm a happy worker!");
        // 通知主线程
        cvar.notify_one();
        let mut index = 1;
        loop {
            if index > 5 {
                break;
            }
            thread::sleep(Duration::from_secs(1));
            println!("working...");
            index += 1;
        }
    });

    println!("abc");

    // 等待工作线程的通知
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("c");
        started = cvar.wait(started).unwrap();
    }
    eprintln!("Worker started!");
}
