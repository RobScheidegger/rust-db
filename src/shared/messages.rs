use std::mem::size_of;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MessageType {
    Add = 0,
    Update = 1,
    Get = 2,
    Delete = 3,
}

pub struct Message {
    pub message_type: MessageType,
    pub key_data: [u8; 50],
    pub value_data: [u8; 50],
}

impl Message {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = [0; size_of::<Message>()];
        let message_type_int: [u8; 1] = [self.message_type as u8];
        buffer[..size_of::<MessageType>()].copy_from_slice(&message_type_int);
        buffer[size_of::<MessageType>()..size_of::<MessageType>() + 50]
            .copy_from_slice(&self.key_data);
        buffer[size_of::<MessageType>() + 50..size_of::<MessageType>() + 100]
            .copy_from_slice(&self.value_data);

        return buffer.to_vec();
    }

    pub fn deserialize(buffer: &[u8]) -> Message {
        let mut message = Message {
            message_type: MessageType::Add,
            key_data: [0; 50],
            value_data: [0; 50],
        };

        let message_type_int: [u8; 1] = [buffer[0]];
        message.message_type = match message_type_int[0] {
            0 => MessageType::Add,
            1 => MessageType::Update,
            2 => MessageType::Get,
            3 => MessageType::Delete,
            _ => MessageType::Add,
        };

        message.key_data.copy_from_slice(&buffer[1..51]);
        message.value_data.copy_from_slice(&buffer[51..101]);

        return message;
    }
}

pub struct Response {
    pub message_type: MessageType,
    pub success: bool,
    pub data: [u8; 50],
}

// Implement serialize and deserialize for response
impl Response {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = [0; size_of::<Response>()];
        let message_type_int: [u8; 1] = [self.message_type as u8];
        buffer[..size_of::<MessageType>()].copy_from_slice(&message_type_int);
        buffer[size_of::<MessageType>()..size_of::<MessageType>() + 1]
            .copy_from_slice(&[self.success as u8]);
        buffer[size_of::<MessageType>() + 1..size_of::<MessageType>() + 51]
            .copy_from_slice(&self.data);

        return buffer.to_vec();
    }

    pub fn deserialize(buffer: &[u8]) -> Response {
        let mut response = Response {
            message_type: MessageType::Add,
            success: false,
            data: [0; 50],
        };

        let message_type_int: [u8; 1] = [buffer[0]];
        response.message_type = match message_type_int[0] {
            0 => MessageType::Add,
            1 => MessageType::Update,
            2 => MessageType::Get,
            3 => MessageType::Delete,
            _ => MessageType::Add,
        };

        response.success = buffer[1] == 1;
        response.data.copy_from_slice(&buffer[2..52]);

        return response;
    }
}
