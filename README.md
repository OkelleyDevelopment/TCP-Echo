# TCP Echo

## Motivation

Build a TCP server with Rust that allows for multiple clients with multithreading.

## Program Compilation and Execution

Assuming Rust lang and cargo are installed:

1. Start the server in one terminal:
   `cd server; cargo run <port>`

2. Start N clients:
   `cd client; cargo run <address> <port>`

## Future Goals

- [] Add feature to allow communication between users
- [] Have server disconnect idle users
