use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let mut rr_index: i32 = 0;
    let result = TcpListener::bind("127.0.0.1:80");

    let listener = match result {
        Ok(l) => l,
        Err(e) => {
            println!("Error : {}", e);
            return;
        }
    };

    for tcp_bind_result in listener.incoming() {
        println!("Request Count: {rr_index}");

        let mut browser_read_stream = match tcp_bind_result {
            Ok(s) => s,
            Err(e) => {
                println!("Error : {e}");
                return;
            }
        };

        let _ = browser_read_stream.set_nonblocking(false);
        let mut browser_write_stream = browser_read_stream
            .try_clone()
            .expect("Failed to clone browser stream");

        let server_address = get_server_address(&mut rr_index);
        let tcp_connect_result = TcpStream::connect(server_address);

        let mut server_read_stream = match tcp_connect_result {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {e}");
                return;
            }
        };

        let _ = server_read_stream.set_nonblocking(false);
        let mut server_write_stream = server_read_stream
            .try_clone()
            .expect("Failed to clone server stream");

        thread::spawn(move || {
            println!(">> Spawning thread_1 - Handling incoming traffic");
            loop {
                handle(&mut server_write_stream, &mut browser_read_stream);
            }
        });

        thread::spawn(move || {
            println!(">> Spawning thread_2 - Handling outgoing traffic");
            loop {
                handle(&mut browser_write_stream, &mut server_read_stream);
            }
        });
    }
}

fn get_server_address(rr_index: &mut i32) -> String {
    let server_addresses = vec![
        String::from("127.0.0.1:8080"),
        String::from("127.0.0.1:8080"),
        String::from("127.0.0.1:8080"),
    ];

    let index = (*rr_index as usize) % server_addresses.len();
    let s = server_addresses.get(index).unwrap().clone();

    println!("pricked server {index} : {s}");
    *rr_index += 1;

    return s;
}

fn handle(write_stream: &mut TcpStream, read_stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    let result = read_stream.read(&mut buffer);

    let n = match result {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    let _ = write_stream.write(&buffer[..n]);
}
