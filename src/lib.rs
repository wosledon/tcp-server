pub mod tcp_server {
    use std::{
        fmt::Display,
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        sync::{Arc, Mutex},
        thread,
    };

    use crate::thread_pool;

    #[derive(Debug)]
    pub struct TcpServer {
        host: String,
        port: i32,
        listener: TcpListener,
        clients: Arc<Mutex<Vec<TcpStream>>>,
    }

    impl Display for TcpServer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TcpServer {{ host: {}, port: {} }}",
                self.host, self.port
            )
        }
    }

    impl TcpServer {
        pub fn new(host: String, port: i32) -> Self {
            let listener = TcpListener::bind(format!("{}:{}", host, port))
                .expect(format!("Failed to bind {}:{}", host, port).as_str());

            println!("[server]-[info]: tcp listening on {}:{}", host, port);

            let clients = Arc::new(Mutex::new(Vec::new()));
            Self {
                host,
                port,
                listener,
                clients,
            }
        }

        pub fn start(self: Arc<Self>, threads: usize) {
            let mut pool = thread_pool::ThreadPool::new(threads);

            for stream in self.listener.incoming() {
                let stream = stream.unwrap();
                let clients = Arc::clone(&self.clients);

                clients.lock().unwrap().push(stream.try_clone().unwrap());

                let self_clone = self.clone();
                pool.threads.push(thread::spawn(move || {
                    self_clone.handle_client(stream);
                }));
            }
        }

        fn handle_client(&self, stream: TcpStream) {
            let mut stream = stream;
            let mut buffer = [0; 1024];

            loop {
                let byte_read = stream.read(buffer.as_mut()).unwrap();

                if byte_read == 0 {
                    return;
                }

                let message = String::from_utf8_lossy(&buffer[..byte_read]);
                println!("[server]-[received]: {message}");

                let clients = self.clients.lock().unwrap();
                for mut client in clients.iter() {
                    client.write_all(message.as_bytes()).unwrap();
                }
            }
        }
    }
}

mod thread_pool {
    use std::thread;

    #[derive(Debug)]
    pub struct ThreadPool {
        pub threads: Vec<thread::JoinHandle<()>>,
    }

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            let mut threads = Vec::with_capacity(size);
            for _ in 0..size {
                threads.push(thread::spawn(|| {
                    println!("[server]-[thread]: Thread spawned");
                }));
            }
            ThreadPool { threads }
        }
    }
}
