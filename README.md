# SinaDB: Minimal Redis clone in Rust
Small Redis clone in rust (FOR LEARNING PURPOSES)

You must have *Rust* installed on your system, if so, git clone this repo then run:
```
cd sinadb
cargo run localhost:8080
```

Use a client (like telnet) to send requests

### Available Requests
- SET: Add new key or update existing one
- GET: Get value from key if it exists
- DEL: Delete key

### Example usage
```
SET name john_doe
SET age 18
SET is_student true

GET name # Returns john_doe
SET age 19 # Updates age to 19
DEL is_student # Deletes key

SET somekey somevalue 10 # Sets somekey for 10 seconds

# Waiting for 10 seconds

GET somekey # ERROR: somekey no longer lives in the cache
```
