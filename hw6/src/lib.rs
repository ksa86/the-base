#![deny(unsafe_code)]

#[cfg(feature = "loom")]
pub mod sync {
    pub use loom::sync::{Arc, Condvar, Mutex};
    pub use loom::thread;
}

#[cfg(not(feature = "loom"))]
pub mod sync {
    pub use std::sync::{Arc, Condvar, Mutex};
    pub use std::thread;
}

use sync::{thread, Arc, Condvar, Mutex};

pub type Task = fn(i64);

struct TaskItem {
    num: i64
}

struct State {
    queue: Vec<TaskItem>,
    shutting_down: bool,
}

struct Shared {
    state: Mutex<State>,
    has_work: Condvar,
}

pub struct ThreadPool {
    shared: Arc<Shared>,
    workers: Vec<thread::JoinHandle<()>>,
    task: Task,
}

impl ThreadPool {
    /// Create a pool with `worker_count` workers.
    ///
    /// # Panics
    ///
    /// Should panic when `worker_count == 0`.
    pub fn new(worker_count: usize, task: Task) -> Self {
        // 1. ThreadPool:new(0, task) -> panic
        if worker_count == 0 {
            panic!("workers count = 0");
        }

        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                queue: Vec::new(),
                shutting_down: false,
            }),
            has_work: Condvar::new(),
        });

        let mut workers = Vec::with_capacity(worker_count);

        // 2. ThreadPool::new(n, task) -> создать n worker-потоков

        for _ in 0..worker_count {
            let shared_clone = Arc::clone(&shared);

            let handle = thread::spawn(move || loop {
                let mut state = shared_clone.state.lock().expect("mutex poisoned");

                // 4. ожидание на condvar
                while state.queue.is_empty() && !state.shutting_down {
                    state = shared_clone.has_work.wait(state).expect("mutex poisoned while waiting");
                }

                // отработка задач перед выходом
                if state.shutting_down && state.queue.is_empty() {
                    break;
                }

                // достаём таск
                if !state.queue.is_empty() {
                    let item = state.queue.remove(0);

                    // освобождаем мьютекс
                    drop(state);

                    // task(item.num)
                    task(item.num);
                }
            });

            workers.push(handle);
        }

        ThreadPool {shared, workers, task }
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        let mut state = self.shared.state.lock().unwrap();

        // 5. не добавлять после shutdown
        if state.shutting_down {
            return;
        }

        state.queue.push(TaskItem { num });

        // разбудить  воркер
        self.shared.has_work.notify_one();
    }

    /// Finish all queued work and stop all workers.
    pub fn shutdown(self) {
        // поставить флаг + разбудить воркеры
        {
            let mut state = self.shared.state.lock().unwrap();
            state.shutting_down = true;
            self.shared.has_work.notify_all();
        }

        // ожидание завершения воркеров
        for worker in self.workers {
            worker.join().expect("worker panicked");
        }
    }
}
