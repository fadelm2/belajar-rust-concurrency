fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::thread;
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
}