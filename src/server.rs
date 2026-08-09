use serde::Deserialize;


#[derive(Clone)]
#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,

    #[serde(skip, default = "default_status")]
    pub status: ServerStatus
}

#[derive(Clone)]
#[derive(Debug)]
pub enum ServerStatus {
    Healthy,
    UnHealthy
}

fn default_status () -> ServerStatus { ServerStatus::UnHealthy }
