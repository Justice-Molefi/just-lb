use std::{collections::HashMap, sync::{Arc, Mutex}};

use reqwest::StatusCode;

use crate::server::{Server, ServerStatus::{Healthy, UnHealthy}};

pub fn check_health(servers: &Arc<Mutex<HashMap<String, Server>>>){
    
    let servers_itr  = servers.lock().unwrap().clone();

    for (key,_) in servers_itr{

        let servers = Arc::clone(&servers);

        tokio::spawn(async move {

            let host;
            let health_endpoint;

            {
                let servers = servers.lock().unwrap();
                let server = servers.get(&key).unwrap();

                host = format!("http://{}:{}", &server.host, &server.port);
                health_endpoint = String::from("Health");
            }

            let result = reqwest::get(format!("{}/{}", host, health_endpoint)).await;
        
            let mut servers = servers.lock().unwrap();
            let mut server = servers.get(&key).unwrap().clone();

            let response = match result {
                Ok(r) => r,
                Err(e) => {
                    println!("Error: {}", e);
                     server.status = UnHealthy;
                    servers.insert(host, server.clone());
                    println!("Server: {}:{} is {:?}", server.host, server.port, server.status);
                    return;
                }
            }; 

            if response.status() == StatusCode::OK {
                server.status = Healthy;
                servers.insert(host, server.clone());
            }else{
                server.status = UnHealthy;
                servers.insert(host, server.clone());
            }

            println!("Server: {}:{} is {:?}", server.host, server.port, server.status);

        });
    }


}
