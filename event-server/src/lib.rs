use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use crate::event_server::EventServer;

mod event_server;

const ADDRESS: &'static str = "127.0.0.1:8080";

#[derive(Debug, bincode::Encode, bincode::Decode)]
enum Events {
    StrEvent(String),
    IntEvent(usize),
}


fn run_client() -> JoinHandle<()> {
    return thread::spawn(|| {
        sleep(Duration::from_secs(3));
        println!("[CLIENT] connecting");
        let mut tcp = TcpStream::connect(ADDRESS).unwrap();
        tcp.write_all(bincode::encode_to_vec(Events::IntEvent(123), bincode::config::standard()).unwrap().as_slice()).unwrap();
        tcp.flush().unwrap();
    });
}


#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>>{

        let server = EventServer::<Events>::builder()
            .on_connect(|conn| println!("[SERVER] Connected {:?}", conn))
            .on_disconnect(|conn| println!("[SERVER] Disconnected {:?}", conn))
            .on_event(|id, event| {
                println!("Got event {event:?}");
                match event {
                    Events::StrEvent(value) => {
                        println!("Str value {value}");
                    }
                    Events::IntEvent(value) => {
                        println!("Str value {value}");
                    }
                }
            })
            .run(ADDRESS);

        run_client();
        run_client();
        println!("Starting server");
        let r = server.await;
        println!("Server stopped");
        return r;

    }
}