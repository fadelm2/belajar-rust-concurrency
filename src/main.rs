use std::thread;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 1..6 {
                println!("Counting ... {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Application finish");
        thread::sleep(Duration::from_secs(7));
    }
    #[test]
    fn test_join_thread() {
        let handle: JoinHandle<i32> = thread::spawn(|| {
            let mut counter = 0;
            for i in 0..=5 {
                println!("Counter: {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }
            return counter;
        });

        println!("Waiting Handle");

        let result = handle.join();
        match result {
            Ok(counter) => println!("Total Counter: {}", counter),
            Err(err) => println!("Error: {:?}", err),
        }
        println!("Application finished");
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        let current = thread::current();
        for i in 1..6 {
            match current.name() {
                None => {
                    println!("{:?} : COunter : {}", current.id(), i);
                }
                Some(name) => {
                    println!("{} : Counter :{}", name, i)
                }
            }
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
        return counter;
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();

        println!("Total Counter 1 {:?}", result1);
        println!("Total Counter 2 {:?}", result2);
        println!("Application finished");
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(|| calculate());
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => {
                println!("Total  Counter : {}", counter);
            }
            Err(error) => {
                println!("Error {:?}", error)
            }
        }
        match result2 {
            Ok(counter) => {
                println!("Total Counter 2 : {}", counter)
            }
            Err(error) => {
                println!("Error : {:?}", error)
            }
        }
        println!("Application Finish");
    }

    #[test]
    fn test_closure() {
        let current_thread = thread::current();
        println!("Current thread: {}", current_thread.name().unwrap());

        let name = String::from("Fadel");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello {}", name);
        };

        let handle = thread::spawn(closure);
        handle.join().unwrap();
    }

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My Thread".to_string());

        let handle = factory.spawn(calculate).expect("Failed to create a new thread");
        let total = handle.join().unwrap();

        println!("Final Total Counter: {:?}", total);
    }
    
    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            sender.send("Hello From Thread".to_string());
        });
        
        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message);
        });
        
        let _ = handle1.join();
        let _ = handle2.join();
    }
}
