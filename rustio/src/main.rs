use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

fn main() {
    //println!("Command-line argument: ");
    //cmd_arg();

    //println!("Standard input: ");
    std_io_01();
    //std_io_02();
    //std_io_03();

    println!("Network I/O code: ");
    //tcp_conn_01();
    //tcp_conn_02();
    //tcp_conn_03();
    //tcp_conn_04();
}

fn cmd_arg() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];
    //let last = &args[args.len()];

    // recommendation
    /* if args.len() < 3 {
        panic!("Invalid number of arguments");
    } else {
        let query = args[1].clone();
        let filename = args[2].clone();
    } */

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

fn std_io_01() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    //io::stdin().read_line(&mut input).unwrap();
    /* io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line"); */

    println!("You typed: {}", input.trim());

    Ok(())
}

fn std_io_02() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let result = input.trim().parse();

    let u_int = match result {
        Ok(num) => num,
        Err(err) => {
            eprintln!("Please type a number!\nError Message: {}", err);
            -1
        }
    };

    println!("Input number: {}", u_int);
}

fn std_io_03() -> io::Result<()> {
    let mut val = String::new();

    io::stdin().read_line(&mut val)?;
    let mut substr_iter = val.split_whitespace();
    let mut next_num = || -> usize {
        substr_iter
            .next()
            .expect("Not enough input numbers")
            .parse()
            .expect("Input is not a number")
    };
    let val1 = next_num();
    let val2 = next_num();

    println!("Values are {} and {}", val1, val2);

    Ok(())
}

fn tcp_conn_01() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }
}

fn tcp_conn_02() {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 7878)),
        SocketAddr::from(([127, 0, 0, 1], 7070)),
    ];
    let listener = TcpListener::bind(&addrs[..]).unwrap();

    println!("Connected to: {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
            }
            Err(e) => { /* connection failed */ }
        }
    }
}

fn tcp_conn_03() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn tcp_conn_04() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_conn_rw(stream);
        });
    }
}

fn handle_conn_rw(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("content.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
