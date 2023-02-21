use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //println!("Basic Closure Code ...");
    //closure_fn();

    //println!("Code for creating new thread besides main thread ...");
    //thread_basics();

    //println!("Code for joining new thread with main thread ...");
    //let handle = thread_join();
    //handle.join().unwrap();

    //cl_fn_once();

    more_closure();

    //println!("Code where new thread is trying to use values of main thread ...");
    //thread_share();

    //println!("Code where new thread is trying to use values of main thread through moving ...");
    //thread_move_share();

    //println!("Code creates a channel, 1 producer, 1 receiver; communicates ...");
    //channel_communication();

    //println!("Code creates a channel, 1 producer, 1 receiver; multiple message communicates ...");
    //channel_multi_communication();

    //println!("Code creates a channel, n producer, 1 receiver; multiple message communicates ...");
    //channel_multi_producer();

    //recv_err();
}

fn closure_fn() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    //generate_workout(simulated_user_specified_value, simulated_random_number);
    //generate_workout_refactor(simulated_user_specified_value, simulated_random_number);
    generate_workout_closure(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn generate_workout_refactor(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

fn generate_workout_closure(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn thread_basics() {
    // following code section is known as closure (anonymous function)
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_join() -> thread::JoinHandle<()> {
    let cl_th = || {
        for i in 1..10 {
            simulated_expensive_calculation(i);
        }
    };

    let handle1 = thread::spawn(cl_th);
    let handle2 = thread::spawn(cl_th);

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle1.join().unwrap();
    handle2
    //handle.join().unwrap();
}

fn thread_share() {
    let v = vec![1, 2, 3];

    /* let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    }); */

    /*  let r = &v;

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", r);
    }); */

    // drop(v);

    //handle.join().unwrap();
}

fn thread_move_share() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    //drop(v);
    //println!("Here's a vector: {:?}", v);

    handle.join().unwrap();
}

fn cl_fn_once() {
    let x = String::from("x");
    let consume_and_return_x = move || x;
    consume_with_relish(consume_and_return_x);
}

fn more_closure() {
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);

        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = /*move*/ || list.push(7);

        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }
}

fn consume_with_relish<F>(func: F)
where
    F: FnOnce() -> String,
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());

    println!("Delicious!");

    //println!("Consumed: {}", func());

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}

fn channel_communication() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn channel_multi_communication() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn channel_multi_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            //println!("{}", val);
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

    //let rx1 = rx.clone();

    for received in rx {
        println!("Got: {}", received);
    }
}

use std::sync::mpsc::RecvError;

fn recv_err() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let vals = [1, 2, 3, 4];

        tx.send(vals[0]);
        tx.send(vals[1]);
        tx.send(vals[2]);
        drop(tx);
    });

    // wait for the thread to join so we ensure the sender is dropped
    handle.join().unwrap();

    rx.recv().unwrap();
    rx.recv().unwrap();
    rx.recv().unwrap();
    //rx.recv().unwrap();
}
