use rustdb::server::node;

fn main() {
    env_logger::init();
    log::info!("Server starting...");
    let node = node::Node::new("Hello", "World");
    let value = node.value;

    log::info!("Starting HTTP server...");
    println!("Server! Value: {value}");
}
