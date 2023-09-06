use rustdb::server::api::handle_message;
use rustdb::server::database::Database;
use rustdb::shared::messages::Message;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

fn handle_client(mut stream: TcpStream, db: Arc<Mutex<Database>>) -> std::io::Result<()> {
    println!("Accepted connection from: {:?}", stream.peer_addr()?);

    let mut buffer = [0; 1024];
    let mut db = db.lock().unwrap();

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        // Serialize the buffer into a Message struct
        let message = Message::deserialize(&buffer[0..bytes_read]);

        let result = handle_message(&message, &mut db);

        let bytes_written = stream.write(&result.serialize());
        match bytes_written {
            Ok(_) => {
                log::info!("Message sent to client ({:?})", result.success);
            }
            Err(m) => {
                log::error!("Error sending message to client: {:?}", m);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Server starting...");

    let db = Arc::new(Mutex::new(Database::new()));

    log::info!("Starting TCP server...");
    let listener = TcpListener::bind("127.0.0.1:8888")?;

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        let stream = stream?;
        let db = Arc::clone(&db);

        // Spawn a new thread to handle each client
        std::thread::spawn(move || {
            handle_client(stream, db).unwrap_or_else(|error| eprintln!("Error: {:?}", error));
        });
    }

    log::info!("Exited TCP server.");

    return Ok(());
}
