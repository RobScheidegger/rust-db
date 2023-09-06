use crate::server::database::Database;
use crate::shared::messages::*;

pub fn handle_message(message: &Message, db: &mut Database) -> Response {
    match message.message_type {
        MessageType::Add => {
            let key = String::from_utf8(message.key_data.to_vec()).unwrap();
            let value = String::from_utf8(message.value_data.to_vec()).unwrap();
            return handle_add(&key, &value, db);
        }
        MessageType::Update => {
            let key = String::from_utf8(message.key_data.to_vec()).unwrap();
            let value = String::from_utf8(message.value_data.to_vec()).unwrap();
            return handle_update(&key, &value, db);
        }
        MessageType::Delete => {
            let key = String::from_utf8(message.key_data.to_vec()).unwrap();
            return handle_delete(&key, db);
        }
        MessageType::Get => {
            let key = String::from_utf8(message.key_data.to_vec()).unwrap();
            return handle_get(&key, db);
        }
    }
}

fn handle_add(key: &String, value: &String, db: &mut Database) -> Response {
    let result = db.add(key, value);
    return Response {
        message_type: MessageType::Add,
        success: result.is_ok(),
        data: [0; 50],
    };
}

fn handle_update(key: &String, value: &String, db: &mut Database) -> Response {
    let result = db.update(key, value);
    return Response {
        message_type: MessageType::Update,
        success: result.is_ok(),
        data: [0; 50],
    };
}

fn handle_delete(key: &String, db: &mut Database) -> Response {
    let result = db.delete(key);
    return Response {
        message_type: MessageType::Delete,
        success: result.is_ok(),
        data: [0; 50],
    };
}

fn handle_get(key: &String, db: &mut Database) -> Response {
    let result = db.get(key);
    let mut response = Response {
        message_type: MessageType::Get,
        success: result.is_ok(),
        data: [0; 50],
    };

    // Copy the result vector into the response.data array
    let result = result.unwrap();
    response.data[..result.len()].copy_from_slice(&result.as_bytes()[..]);

    return response;
}
