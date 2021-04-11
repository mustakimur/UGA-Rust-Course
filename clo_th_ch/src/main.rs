use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //println!("Basic Closure Code ...");
    //closure_fn();

    //println!("Code for creating new thread besides main thread ...");
    //thread_basics();

    //println!("Code for creating new thread besides main thread ...");
    //thread_basics();

    //println!("Code for joining new thread with main thread ...");
    //thread_join();

    println!("Code where new thread is trying to use values of main thread ...");
    thread_share();

    //println!("Code where new thread is trying to use values of main thread through moving ...");
    //thread_move_share();

    //println!("Code creates a channel, 1 producer, 1 receiver; communicates ...");
    //channel_communication();

    //println!("Code creates a channel, 1 producer, 1 receiver; multiple message communicates ...");
    //channel_multi_communication();

    //println!("Code creates a channel, n producer, 1 receiver; multiple message communicates ...");
    //channel_multi_producer();
}

fn closure_fn() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    //generate_workout_refactor(simulated_user_specified_value, simulated_random_number);
    //generate_workout_closure(simulated_user_specified_value, simulated_random_number);
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
    let expensive_closure = |num| {
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

fn thread_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
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
}
