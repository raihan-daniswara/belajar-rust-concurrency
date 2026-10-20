use std::thread;

fn main() {
    let current_thread = thread::current();
    println!("{} : Hello, world!", current_thread.name().unwrap());
}

#[cfg(test)]
mod tests {
    use super::thread;
    use std::{
        cell::{Cell, RefCell},
        sync::{Arc, Barrier, Once},
        time::Duration,
    };

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 0..=5 {
                println!("Counter = {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Application Finished");

        thread::sleep(Duration::from_secs(7));
    }

    #[test]
    fn test_join_thread() {
        let handle = thread::spawn(|| {
            let mut counter = 0;
            for i in 0..=5 {
                println!("counter = {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1
            }
            return counter;
        });

        println!("Waiting Handle");

        let result = handle.join();

        match result {
            Ok(counter) => println!("Total Counter : {}", counter),
            Err(error) => println!("Error : {:?}", error),
        }

        println!("Application Finished");
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        let current = thread::current();
        for i in 0..=5 {
            match current.name() {
                Some(name) => println!("{} : counter = {}", name, i),
                None => println!("{:?} : counter = {}", current.id(), i),
            }
            println!("counter = {}", i);
            thread::sleep(Duration::from_secs(1));
            counter += 1
        }
        return counter;
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();

        println!("Total Counter 1 = {}", result1);
        println!("Total Counter 2 = {}", result2);
        println!("Application Finished")
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(|| calculate());
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => println!("Total Counter 1 = {}", counter),
            Err(error) => println!("Error = {:?}", error),
        }

        match result2 {
            Ok(counter) => println!("Total Counter 2 = {}", counter),
            Err(error) => println!("Error = {:?}", error),
        }

        println!("Application Finished")
    }

    #[test]
    fn test_closure() {
        let current_thread = thread::current();
        println!("thread name = {}", current_thread.name().unwrap());

        let name = String::from("Raihan Daniswara");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello, {}!", name)
        };

        // error karena scope variable nya sudah dipindahkan dengan move
        // println!("Hello, {}!", name)

        let handle = thread::spawn(closure);
        handle.join().unwrap()
    }

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My Thread".to_string());

        let handle = factory
            .spawn(calculate)
            .expect("Failed to create a new thread");

        let total = handle.join().unwrap();

        println!("Total Counter = {}", total)
    }

    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            sender.send("Hello from thread".to_string())
        });

        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message)
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_queue() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello From thread".to_string());
            }
            sender.send("Exit".to_string())
        });

        let handle2 = thread::spawn(move || {
            loop {
                let message = receiver.recv().unwrap();
                if message == "Exit" {
                    break;
                }
                println!("{}", message)
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from thread".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            loop {
                for message in receiver.iter() {
                    println!("{}", message)
                }
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let sender2 = sender.clone();

        let handle1 = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from sender 1".to_string());
            }
        });

        let handle3 = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_secs(3));
                sender2.send("Hello from sender 2".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            loop {
                for message in receiver.iter() {
                    println!("{}", message)
                }
            }
        });

        let _ = handle1.join();
        let _ = handle3.join();
        let _ = handle2.join();
    }

    static mut COUNTER: i32 = 0;

    #[test]
    fn test_race_condition() {
        let mut handles = vec![];
        for _ in 0..10 {
            let handle = thread::spawn(|| unsafe {
                for i in 0..1000000 {
                    COUNTER += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap()
        }

        println!("Counter = {}", unsafe { COUNTER })
    }

    #[test]
    fn test_atomic() {
        use std::sync::atomic::{AtomicI64, Ordering};

        static counter: AtomicI64 = AtomicI64::new(0);
        let mut handlers = vec![];

        for _ in 0..10 {
            let handler = thread::spawn(|| {
                for _ in 0..1000000 {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap()
        }

        println!("Counter: {}", counter.load(Ordering::Relaxed))
    }

    #[test]
    fn test_atomic_reference() {
        use std::sync::atomic::{AtomicI64, Ordering};

        let counter: Arc<AtomicI64> = Arc::new(AtomicI64::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                for _ in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap()
        }

        println!("Counter: {}", counter.load(Ordering::Relaxed))
    }

    #[test]
    fn test_mutex() {
        use std::sync::{Arc, Mutex};

        let counter: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                for _ in 0..1000000 {
                    let mut data = counter_clone.lock().unwrap();
                    *data += 1;
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap()
        }

        println!("Counter: {}", *counter.lock().unwrap())
    }

    thread_local! {
        pub static NAME: RefCell<String> = RefCell::new("Default".to_string())
    }
    thread_local! {
        pub static OTHER_NAME: RefCell<String> = RefCell::new("Default".to_string())
    }

    #[test]
    fn test_thread_local() {
        let handler = thread::spawn(|| {
            NAME.with_borrow_mut(|name| *name = "Budi".to_string());

            NAME.with_borrow(|name| println!("Hello, {}", name));
        });

        handler.join().unwrap();

        {
            NAME.with_borrow_mut(|name| *name = "Andi".to_string());
            NAME.with_borrow(|name| println!("Hello, {}", name))
        }

        NAME.with_borrow(|name| println!("Hello, {}", name))
    }

    #[test]
    fn test_thread_panic() {
        let handle = thread::spawn(|| panic!("Oops! Something went wrong!"));

        match handle.join() {
            Ok(_) => println!("Thread Finished"),
            Err(_) => println!("Thread Panic"),
        }

        println!("Application Finished")
    }

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10));
        let mut handlers = vec![];

        for i in 0..10 {
            let barrier_clone = Arc::clone(&barrier);
            let handler = thread::spawn(move || {
                println!("Join Gamer-{}", i);
                barrier_clone.wait();
                println!("Gamer-{} Start", i)
            });

            handlers.push(handler)
        }

        for handler in handlers {
            handler.join().unwrap()
        }
    }

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INIT: Once = Once::new();

    fn get_total() -> i32 {
        unsafe {
            TOTAL_INIT.call_once(|| {
                println!("Call Once");
                TOTAL_COUNTER += 1
            });
            TOTAL_COUNTER
        }
    }

    #[test]
    fn test_once() {
        let mut handlers = vec![];

        for _ in 0..10 {
            let handler = thread::spawn(move || {
                let total = get_total();
                println!("Total: {}", total)
            });

            handlers.push(handler)
        }

        for handler in handlers {
            handler.join().unwrap()
        }
    }
}
