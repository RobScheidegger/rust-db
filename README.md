# rust-db

A basic concurrent tree-based database implemented in Rust. 

## Requirements

The client should be able to:

1. Perform all of the commands below in a REPL.

The server should be able to:

1. Maintain a BST of some values.
2. Respond to the following queries:

    - Insert {Key: str, Value: str}
    - Delete {Key: str}
    - Update {Key: str, Value: str}
    - Get {Key: str} 