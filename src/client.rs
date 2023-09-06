use rustdb::shared::messages::{Message, MessageType, Response};
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::TcpStream;

fn construct_message(input: &String) -> Option<Message> {
    let mut message = Message {
        message_type: MessageType::Add,
        key_data: [0; 50],
        value_data: [0; 50],
    };

    let mut input_iter = input.split_whitespace();
    let command = match input_iter.next() {
        Some(x) => {
            if !(x == "add" || x == "update" || x == "delete" || x == "get") {
                print!("Invalid command name");
                return None;
            }
            x
        }
        None => {
            print!("Command required.");
            return None;
        }
    };
    let key = match input_iter.next() {
        Some(x) => x,
        None => "",
    };
    let key_len = key.len();

    if key_len == 0 {
        print!("Key is required");
        return None;
    }

    if key_len > 50 {
        print!("Key is too long");
        return None;
    }

    let value = match input_iter.next() {
        Some(x) => x,
        None => "",
    };
    let value_len = value.len();

    if value.len() > 50 {
        print!("Value is too long");
        return None;
    }

    match command {
        "add" => {
            message.message_type = MessageType::Add;
            message.key_data[..key_len].copy_from_slice(key.as_bytes());
            if value.len() == 0 {
                print!("Value is required");
                return None;
            }
            message.value_data[..value_len].copy_from_slice(value.as_bytes());
        }
        "update" => {
            message.message_type = MessageType::Update;
            message.key_data[..key_len].copy_from_slice(key.as_bytes());
            if value.len() == 0 {
                print!("Value is required");
                return None;
            }
            message.value_data[..value_len].copy_from_slice(value.as_bytes());
        }
        "delete" => {
            message.message_type = MessageType::Delete;
            message.key_data[..key_len].copy_from_slice(key.as_bytes());
        }
        "get" => {
            message.message_type = MessageType::Get;
            message.key_data[..key_len].copy_from_slice(key.as_bytes());
        }
        _ => {
            print!("Invalid command");
            return None;
        }
    }

    return Some(message);
}

fn main() {
    // Implement a repl for sending messages
    let mut input = String::new();
    let mut stream = TcpStream::connect("127.0.0.1:8888");

    if stream.is_err() {
        println!("Error connecting to server.");
        return;
    }

    println!("Connected to server.");

    loop {
        print!("\n> ");
        // Make sure to flush the stdout buffer
        if std::io::stdout().flush().is_err() {
            println!("Error flushing stdout");
            return;
        }

        let result = std::io::stdin().read_line(&mut input);
        if result.is_err() {
            println!("Error reading input.");
            return;
        }

        let message = construct_message(&input);
        // Send the message to the server
        if message.is_none() {
            continue;
        }

        let stream = stream.as_mut().unwrap();
        let result = stream.write(&message.unwrap().serialize());
        match result {
            Ok(_) => {
                println!("Message sent.");
            }
            Err(_) => {
                println!("Error sending message.");
            }
        }
        let mut response_buffer = [0 as u8; size_of::<Response>()];
        let response = stream.read_exact(&mut response_buffer);
        match response {
            Err(_) => {
                println!("Error reading response.");
            }
            Ok(_) => {
                let response = Response::deserialize(&response_buffer);
                println!("Response: {:?}", response.success);

                if response.success && response.message_type == MessageType::Get {
                    println!(
                        "Data: {:?}",
                        String::from_utf8(response.data.to_vec())
                            .unwrap()
                            .trim_matches('\0')
                    );
                }
            }
        }

        input.clear();
    }
}
