use std::{
    collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread
};

use rand::RngExt;


fn main() {

    let mut servers: HashMap<String, String> = HashMap::new();
    
    servers.insert("127.0.0.1:8080".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8081".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8082".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8083".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8084".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8085".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8086".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8087".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8088".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8089".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8090".to_string(), "unhealthy".to_string());
    servers.insert("127.0.0.1:8091".to_string(), "unhealthy".to_string());


    check_health(servers);

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

    println!("picked server {index} : {s}");
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


fn check_health(servers: HashMap<String,String>){

    let servers_itr = servers.clone();
    let servers = Arc::new(Mutex::new(servers));

    for (addr, _) in servers_itr{

        let servers = Arc::clone(&servers);

        thread::spawn(move ||{
            let r = rand::rng().random_range(0..2);
            
            let mut servers = servers.lock().unwrap();

            if r == 0{
                servers.insert(addr.clone(), "healthy".to_string());
                println!("Server: {addr}, Status: healthy");
            }else{
                servers.insert(addr.clone(), "unhealthy".to_string());
                println!("Server: {addr}, Status: unhealthy");
            }
            
        });
    }

}
