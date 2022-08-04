use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use bincode::{Decode, Encode};
use bincode::config::Configuration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

pub struct EventServer<T: Decode + Encode> {
    config: Configuration,
    max_event_size: usize,
    connections: Arc<Connections<T>>,
    on_connect: Arc<dyn Fn(&Connection<T>) + Send + Sync>,
    on_disconnect: Arc<dyn Fn(&Connection<T>) + Send + Sync>,
    on_event: Arc<dyn Fn(usize, T) + Send + Sync>,
}

type Tx<T> = mpsc::UnboundedSender<T>;
type Rx<T> = mpsc::UnboundedReceiver<T>;

struct Connections<T> {
    counter: AtomicUsize,
    peers: RwLock<HashMap<usize, Connection<T>>>,
}

impl<T> Connections<T> {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
            peers: Default::default(),
        }
    }

    pub fn insert(&self, address: SocketAddr, tx: Tx<T>) -> usize {
        let mut lock = self.peers.write().unwrap();
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        lock.insert(id, Connection {
            id,
            tx,
            address,
        });
        return id;
    }

    pub fn remove(&self, conn_id: usize) -> Option<Connection<T>> {
        let mut lock = self.peers.write().unwrap();
        return lock.remove(&conn_id);
    }

    pub fn send_to(&self, conn_id: usize, data: T) -> Result<(), SendError<T>> {
        if let Ok(mut lock) = self.peers.read() {
            match lock.get(&conn_id) {
                None => {
                    println!("No connection with id {conn_id}");
                    return Err(SendError(data));
                }
                Some(conn) => {
                    return conn.send(data);
                }
            }
        }
        return Err(SendError(data));
    }
}

#[derive(Debug)]
pub struct Connection<T> {
    id: usize,
    tx: Tx<T>,
    address: SocketAddr,
}

impl<T> Connection<T> {
    pub fn send(&self, data: T) -> Result<(), SendError<T>> {
        return self.tx.send(data);
    }
}


impl<T> EventServer<T>
    where
        T: Decode + Encode + Send + 'static
{
    pub fn builder() -> Self {
        Self {
            max_event_size: 1024,
            config: bincode::config::standard(),
            connections: Arc::new(Connections::new()),
            on_connect: Arc::new(|_| {}),
            on_disconnect: Arc::new(|_| {}),
            on_event: Arc::new(|_, _| {}),
        }
    }

    pub fn bincode_config<F>(mut self, config: Configuration) -> Self {
        self.config = config;
        return self;
    }

    pub fn max_event_size<F>(mut self, max_event_size: usize) -> Self {
        self.max_event_size = max_event_size;
        return self;
    }

    pub fn on_connect<F>(mut self, on_connect: F) -> Self
        where
            F: Fn(&Connection<T>) + 'static + Send + Sync
    {
        self.on_connect = Arc::new(on_connect);
        return self;
    }

    pub fn on_disconnect<F>(mut self, on_disconnect: F) -> Self
        where
            F: Fn(&Connection<T>) + 'static + Send + Sync
    {
        self.on_disconnect = Arc::new(on_disconnect);
        return self;
    }


    pub fn on_event<F>(mut self, on_event: F) -> Self
        where
            F: Fn(usize, T) + 'static + Send + Sync
    {
        self.on_event = Arc::new(on_event);
        return self;
    }

    pub async fn run<A: ToSocketAddrs>(self, addr: A) -> Result<(), Box<dyn std::error::Error>> {

        let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel::<()>();

        let listener = TcpListener::bind(addr).await?;

        loop {
            let (mut stream, address) = listener.accept().await?;


            let on_connect = Arc::clone(&self.on_connect);
            let on_event = Arc::clone(&self.on_event);
            let on_disconnect = Arc::clone(&self.on_disconnect);

            let (tx, mut rx) = mpsc::unbounded_channel();

            //TODO parameter
            let mut write_buf = vec![0u8; self.max_event_size].into_boxed_slice();

            let conn_id = self.connections.insert(address, tx);
            let connections = Arc::clone(&self.connections);

            tokio::spawn(async move {
                if let Ok(map) = connections.peers.read() {
                    if let Some(conn) = map.get(&conn_id) {
                        on_connect(conn);
                    }
                }
                let mut buf = vec![0; self.max_event_size].into_boxed_slice();
                // In a loop, read data from the socket and write the data back.
                loop {
                    match stream.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => {
                            if let Some(conn) = connections.remove(conn_id) {
                                on_disconnect(&conn)
                            }
                        }
                        Ok(n) => {
                            //TODO use framed messages
                            match bincode::decode_from_slice(&buf[..n], self.config) {
                                Ok((event, _)) => {
                                    on_event(conn_id, event);
                                }
                                Err(e) => {
                                    eprintln!("Failed to deserialize event: {}", e)
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            if let Some(conn) = connections.remove(conn_id) {
                                on_disconnect(&conn)
                            }
                            return;
                        }
                    };

                    match rx.try_recv() {
                        Ok(event) => {
                            let size = bincode::encode_into_slice(event, &mut *write_buf, self.config).unwrap();
                            if let Err(e) = stream.write_all(&write_buf[..size]).await {
                                eprintln!("failed to write to socket; err = {:?}", e);
                            }
                        }
                        Err(_) => {}
                    }

                    // //Write the data back
                    // if let Err(e) = stream.write_all(&buf[0..n]).await {
                    //     eprintln!("failed to write to socket; err = {:?}", e);
                    //     return;
                    // }
                }
            });
        }
    }
}