use std::{thread, time::Duration};
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    spawnando_threads();
    controlando_as_threads_com_handle();
    comunicando_entre_threads();
    sharing_state_with_mutex();
}

#[allow(dead_code)]
fn spawnando_threads () {
    // O código para quando a thread na main finaliza
    thread::spawn(|| {
        for i in 1..=10 {
            println!("Hi, number {} from the thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("Hi, number {} from the main", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[allow(dead_code)]
fn controlando_as_threads_com_handle (){
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("Hi, number {} from the thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("Hi, number {} from the main", i);
        thread::sleep(Duration::from_millis(2));
    }

    // Aguarda a thread dentro do handler finalizar.
    handle.join().unwrap();
}

#[allow(dead_code)]
fn comunicando_entre_threads (){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=10 {
            let msg = String::from(format!("Hi, number {} from the thread", i));
            println!("Send: {}", &msg);
            tx.send(msg).unwrap();
        }
    });

    for _i in 1..=5 {
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
}

#[allow(dead_code)]
fn sharing_state_with_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for i in 0..10 {
        // Cria um clone do contador para poder transferir na clausura
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("Rodando thread {}", i+1);
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Contador: {}", counter.lock().unwrap());
}