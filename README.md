# ConnX


## Overview
This project implements a client-server application in Rust for forwarding client-side web applications to a server via port forwarding. It allows users to access web applications hosted on client devices remotely through a centralized server.

## Features
- Port forwarding to establish a secure tunnel between client and server.
- Remote access to client-side web applications from the server.

[ TODO ] Dynamic routing of incoming requests to the appropriate client-side application.

[ TODO ] Encryption and authentication mechanisms for secure communication.


## Getting Started
1. Clone the repository: `git clone https://github.com/your/repository.git`
2. Configure the server with port forwarding rules and routing configurations.
3. Install and configure the client-side software on devices hosting web applications.
4. Access the client-side web application through the server's forwarded port.

## Usage
```bash
# Run the server
cargo run --bin server

# Run the client
cargo run --bin client
```
