use rustdb::shared::message;
use rustdb::shared::messages::insert_message::InsertMessage;

fn main() {
    println!("I am the client! AHHHH!");
    let result: i32 = message::sum(1, 2);
    let insert_message = InsertMessage {
        key: "Hello world!".to_string(),
        value: "Test".to_string(),
    };

    // let insert_message_2 = InsertMessage::new("InsertMessage".to_string(), "test".to_string());
    let test = Box::new(insert_message);
    let t = test.value;
    println!("Result: {result}");
    println!("Hi! {t}");
}
