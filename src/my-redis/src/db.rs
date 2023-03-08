use bytes::Bytes;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Notify,
};

#[derive(Debug)]
pub(crate) struct DbDropGuard {
    db: Db,
}

#[derive(Debug, Clone)]
pub(crate) struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}

#[derive(Debug)]
struct State {
    //实体
    entries: HashMap<String, Entry>,
    pub_sub: HashMap<String, Sender<Bytes>>,
    expirations: BTreeMap<(Instant, u64), String>,
    next_id: u64,
    shutdown: bool,
}

#[derive(Debug)]
/// 存储实体
struct Entry {
    id: u64,
    data: Bytes,
    expires_at: Option<Instant>,
}

impl DbDropGuard {}
impl Db {
    pub(crate) fn new() -> Self {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
                pub_sub: HashMap::new(),
                expirations: BTreeMap::new(),
                next_id: 0,
                shutdown: false,
            }),
            background_task: Notify::new(),
        });
        //这里需要启动一个后台任务 TODO:
        Self { shared }
    }
    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|s| s.data.clone())
    }

    pub(crate) fn set(&self, key: String, value: Bytes, expires: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();
        let id = state.next_id;
        state.next_id += 1;
        let mut notify = false;
        let expires_at = expires.map(|duration| {
            let when = Instant::now() + duration;

            // TODO:不懂
            notify = state
                .next_expiration()
                .map(|exp| exp > when)
                .unwrap_or(true);
            state.expirations.insert((when, id), key.clone());
            when
        });
        let prev = state.entries.insert(
            key,
            Entry {
                id,
                data: value,
                expires_at,
            },
        );
        if let Some(prev) = prev {
            if let Some(s) = prev.expires_at {
                //删除重复的
                state.expirations.remove(&(s, prev.id));
            }
        }
        drop(state);
        if notify {
            // TODO: 通知
            todo!()
        }
    }

    pub(crate) fn subscribe(&self, key: String) -> Receiver<Bytes> {
        use std::collections::hash_map::Entry;
        let mut state = self.shared.state.lock().unwrap();

        match state.pub_sub.entry(key) {
            Entry::Occupied(o) => o.get().subscribe(),
            Entry::Vacant(v) => {
                let (tx, rx) = broadcast::channel(1024);
                v.insert(tx);
                rx
            }
        }
    }
}

impl State {
    fn next_expiration(&self) -> Option<Instant> {
        self.expirations.keys().next().map(|s| s.0)
    }
}
