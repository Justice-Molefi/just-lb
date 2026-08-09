mod checker;
mod config;
mod server;

use std::{
    collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread, time::Duration
};

use tokio::time::{MissedTickBehavior, interval};

use crate::{checker::check_health, config::get_servers, server::Server};

#[tokio::main]
async fn main() {

    // Please remember to fix this soup...
    let servers_vesc: Vec<Server>  = get_servers();
    let servers_vec = servers_vesc.clone();


    let servers: HashMap<String, Server> = servers_vec.into_iter().map(|server| (format!("{}:{}", server.host.clone() , server.port), server)).collect();
    let servers = Arc::new(Mutex::new(servers));

    let servers = Arc::clone(&servers);

    tokio::spawn( async move {
        let mut periodic_interval = interval(Duration::from_secs(5)); 
       
        periodic_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            periodic_interval.tick().await;

            check_health(&servers);
        }
    });


    
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

        let server_address = get_server_address(&mut rr_index, &servers_vesc);
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


fn get_server_address(rr_index: &mut i32, servers: &Vec<Server>) -> String {

    let index = (*rr_index as usize) % servers.len();
    let s = servers.get(index).unwrap().clone();

    println!("picked server {index} : {}", s.host);
    *rr_index += 1;

    let server_host= s.host.clone();
    let server_port: String = s.port.to_string().clone();

    let server_addr =  format!("{server_host}:{server_port}");
    return server_addr;
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
