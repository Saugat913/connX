use clap::{command, Arg, Command};
mod client;
mod config;
mod server;

fn parse_command_line() -> config::Config {
    let matches = command!()
        .subcommand_value_name("mode")
        .subcommand_required(true)
        .about("Connection tunneler")
        .after_help("Be aware to use server address as <ip:port>\nfor example\n     localhost:8080")
        .subcommand(
            Command::new("client")
                .long_flag("client")
                // .subcommand_required(true)
                .arg(
                    Arg::new("service_port")
                        .long("service-port")
                        .help("Port of the service you want to bypass")
                        .required(true),
                )
                .arg(
                    Arg::new("server_address")
                        .long("server-address")
                        .help("Server Address in <ip:port_address> format")
                        .default_value("localhost:8080"),
                )
                .about("Set the mode of program as client"),
        )
        .subcommand(
            Command::new("server")
                .long_flag("server")
                .about("Set the mode of program as server")
                .arg(
                    Arg::new("client_port")
                        .long("client-port")
                        .help("Set the port where client is accepted")
                        .default_value("8080"),
                )
                .arg(
                    Arg::new("customer_port")
                        .long("customer-port")
                        .help("Set the port where customer is accepted")
                        .default_value("8081"),
                ),
        )
        .subcommand_help_heading("mode")
        .get_matches();

    if let Some(server_sub_command) = matches.subcommand_matches("server") {
        let client_port = server_sub_command
            .get_one::<String>("client_port")
            .unwrap()
            .to_owned();

        let customer_port = server_sub_command
            .get_one::<String>("customer_port")
            .unwrap()
            .to_owned();

        return config::Config::ServerConfig(config::ServerConfig {
            client_port: client_port,
            customer_port: customer_port,
        });
    }

    let client_sub_command = matches.subcommand_matches("client").unwrap();

    let service_port = client_sub_command
        .get_one::<String>("service_port")
        .unwrap()
        .to_owned();

    let server_addr = client_sub_command
        .get_one::<String>("server_address")
        .unwrap()
        .to_owned();

    config::Config::ClientConfig(config::ClientConfig {
        service_port: service_port,
        server_address: server_addr,
    })
}

#[tokio::main]
async fn main() {
    let config = parse_command_line();

    match config {
        config::Config::ClientConfig(client_config) => {
            println!(
                "Client :\n Service Port:{} \n Server Address: {}",
                client_config.service_port, client_config.server_address
            );

            let handler = tokio::spawn(client::run_as_client(
                client_config.service_port,
                client_config.server_address,
            ));

            handler.await.unwrap();
        }
        config::Config::ServerConfig(server_config) => {
            println!(
                "Server: \n Client Port: {} \n Customer Port: {}",
                server_config.client_port, server_config.customer_port
            );

            let handler = tokio::spawn(server::run_as_server(
                server_config.client_port,
                server_config.customer_port,
            ));

            handler.await.unwrap();
        }
    }
}
