use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
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

    for stream in listener.incoming() {
        //stream from browser
        let more_streams = match stream {
            Ok(ts) => Arc::new(Mutex::new(ts)),
            Err(e) => {
                println!("Error : {}", e);
                return;
            }
        };

        //open stream to the api server
        let to_server = TcpStream::connect("127.0.0.1:8080");

        //stream to api server
        let to_server_stream = match to_server {
            Ok(tss) => Arc::new(Mutex::new(tss)),
            Err(e) => {
                println!("Some more bullshit {e}");
                return;
            }
        };

        let to_server_stream_t2 = Arc::clone(&to_server_stream);
        let more_streams_t2 = Arc::clone(&more_streams);

        println!("Mark 1");

        thread::spawn(move || {
            println!(">> Spawning thread_1");

            loop {
                //thread 1
                let mut to_server_stream_guard = to_server_stream.lock().unwrap();
                let mut more_streams_guard = more_streams.lock().unwrap();

                more_streams_guard
                    .set_nonblocking(true)
                    .expect("set_nonblocking call failed");

                //buffer to read from browser
                let mut buffer = [0; 1024];

                //read from browser stream while there's bytes
                let res: Result<usize, std::io::Error> = more_streams_guard.read(&mut buffer);

                let n = match res {
                    Ok(r) => r,
                    Err(e) => {
                        println!("{e}");
                        return;
                    }
                };

                println!("Bytes In buffer from client: {n}");

                //write bytes from browser to api server streaM
                let _ = to_server_stream_guard.write(&buffer[..n]);

            }
        });

        println!("Mark 2");

        thread::spawn(move || {
            println!(">> Spawning thread_2");

            loop {
                //thread 2
                let mut to_server_stream_guard2 = to_server_stream_t2.lock().unwrap();
                let mut more_streams_guard2 = more_streams_t2.lock().unwrap();

                // to_server_stream_guard2
                //     .set_nonblocking(true)
                //     .expect("set_nonblocking call failed");


                //buffer to read from api server
                let mut server_buff = [0; 1024];

                //Read bytes from api server
                let res_from_server = to_server_stream_guard2.read(&mut server_buff);
                let n2 = match res_from_server {
                    Ok(r) => r,
                    Err(e) => {
                        println!("{e}");
                        return;
                    }
                };

                println!("Bytes In buffer from server: {n2}");

                //write bytes from api server to browser
                let _ = more_streams_guard2.write(&server_buff[..n2]);
             
                let message = String::from_utf8_lossy(&server_buff[..n2]);
                println!("{message}");
            }
        });
    }
}
