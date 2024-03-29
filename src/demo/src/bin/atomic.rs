use std::cell::RefCell;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(Lock::new(0));
    let data1 = data.clone();

    let t1 = thread::spawn(move || data1.lock(|v| *v += 10));
    let data2 = data.clone();
    let t2 = thread::spawn(move || data2.lock(|v| *v *= 10));
    t1.join().unwrap();
    t2.join().unwrap();
    println!("data: {:?}", data)
}

struct Lock<T> {
    locked: AtomicBool,
    data: RefCell<T>,
}

impl<T> fmt::Debug for Lock<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }
    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        //没有释放锁，循环等待
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) == true {}
        }
        op(&mut self.data.borrow_mut());
        //释放锁
        self.locked.store(false, Ordering::Release)
    }
}
