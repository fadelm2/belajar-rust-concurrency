use std::thread;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 1..6{
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
                counter += 1 ;
            }
            return counter;
        });

        println!("Waiting Handle");

        let result = handle.join();
        match result {
            Ok(counter) => println!("Total Counter: {}", counter),
            Err(err)=> println!("Error: {:?}", err),
        }
        println!("Application finished");
    }
}