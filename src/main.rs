#![allow(unused_doc_comments,unused_imports)]
use std::{
    rc::Rc, sync::{mpsc, Arc, Mutex}, thread, time::Duration
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    // MAIN
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    ///
    ///
    ///
    ///
    ///
    ///
    ///
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();

        *num = 6;

    }

    println!("m = {:#?}", m);

    ///
    ///
    ///
    ///
    ///
    ///
    ///
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    ///
    ///
    ///
    ///
    ///
    ///
    ///
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        for i in 0..10 {
            println!("spawned thread {}", i);
            thread::sleep(Duration::from_millis(2))
        }
    });

    for i in 0..10 {
        println!("main thread {}", i);
        thread::sleep(Duration::from_millis(1))
    }
    handle.join().unwrap();
}
