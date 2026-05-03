use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let result = TcpListener::bind("127.0.0.1:80");

    let listener = match result {
        Ok(l) => l,
        Err(e) => {
            println!("Error : {}", e);
            return;
        }
    };

    for tcp_bind_result in listener.incoming() {
      
        let mut browser_read_stream = match tcp_bind_result {
            Ok(s) => s,
            Err(e) => {
                println!("Error : {e}");
                return;
            }
        };

        let _ = browser_read_stream.set_nonblocking(false);
        let mut browser_write_stream = browser_read_stream.try_clone().expect("Failed to clone browser stream");


        let tcp_connect_result = TcpStream::connect("127.0.0.1:8080");
        let mut server_read_stream = match tcp_connect_result {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {e}");
                return;
            }
        };

        let _ = server_read_stream.set_nonblocking(false);
        let mut server_write_stream = server_read_stream.try_clone().expect("Failed to clone server stream");


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





fn handle(write_stream: &mut TcpStream, read_stream: &mut TcpStream){

    println!("handler running...");

    let mut buffer = [0;1024];

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
