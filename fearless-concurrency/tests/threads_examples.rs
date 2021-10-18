#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};
    use std::sync::{mpsc, Mutex, Arc};

    #[test]
    #[should_panic]
    fn thread_spawn_panic_example() {
        let thread_values = Arc::new(Mutex::new(Vec::new()));
        let mut main_values = Vec::new();
        
        let cloned_thread_values = Arc::clone(&thread_values);
        thread::spawn(move || {
            for i in 1..=10 {
                cloned_thread_values.lock().unwrap().push(i);
                thread::sleep(Duration::from_millis(1));
            }
        });
            
        for i in 1..=5 {
            let thread_values = Arc::clone(&thread_values);
            main_values.push(i);
            thread::sleep(Duration::from_millis(1));

            if i == 5 {
                // Sould Panic!
                println!("{}", thread_values.lock().unwrap()[7]);
            }
        }
    }

    #[test]
    fn thread_spawn_normal_example() {
        let thread_values = Arc::new(Mutex::new(Vec::new()));
        let mut main_values = Vec::new();

        let cloned_thread_values = Arc::clone(&thread_values);
        thread::spawn(move || {
            for i in 1..=10 {
                cloned_thread_values.lock().unwrap().push(i);
                thread::sleep(Duration::from_millis(1));
            }
        });
            
        for i in 1..=5 {
            let thread_values = Arc::clone(&thread_values);
            main_values.push(i);
            thread::sleep(Duration::from_millis(1));

            if i == 5 {
                assert_eq!(thread_values.lock().unwrap()[0], 1);
                assert_eq!(thread_values.lock().unwrap()[1], 2);
                assert_eq!(thread_values.lock().unwrap()[2], 3);
                assert_eq!(thread_values.lock().unwrap()[3], 4);

                // A partir do 5 já não há garantia de que não haverá pânico
            }
        }
    }

    #[test]
    fn thread_handling_example() {
        let thread_values = Arc::new(Mutex::new(Vec::new()));
        let mut main_values = Vec::new();

        let cloned_thread_values = Arc::clone(&thread_values);
        let handler = thread::spawn(move || {
            for i in 1..=10 {
                cloned_thread_values.lock().unwrap().push(i);
                thread::sleep(Duration::from_millis(1));
            }
        });
            
        for i in 1..=5 {
            main_values.push(i);
            thread::sleep(Duration::from_millis(1));
        }

        handler.join().unwrap();
        assert_eq!(thread_values.lock().unwrap()[9], 10);
    }

    #[test]
    fn transmitindo_valores_entre_threads (){
        let (tx, rx) = mpsc::channel();
        let mut received_values = Vec::new();

        thread::spawn(move || {
            for i in 1..=5 {
                let msg = String::from(format!("Hi, number {} from the thread", i));
                tx.send(msg).unwrap();
            }
        });

        for _ in 1..=5 {
            let received = rx.recv().unwrap();
            received_values.push(received);
        }

        assert_eq!(received_values[0], "Hi, number 1 from the thread");
        assert_eq!(received_values[1], "Hi, number 2 from the thread");
        assert_eq!(received_values[2], "Hi, number 3 from the thread");
        assert_eq!(received_values[3], "Hi, number 4 from the thread");
        assert_eq!(received_values[4], "Hi, number 5 from the thread");
    }
}