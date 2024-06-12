use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    select,
    sync::Mutex,
};

pub async fn run_as_client(service_port: String, server_address: String) {
    const LOCAL_ADDR: &str = "127.0.0.1";

    let server_connection = TcpStream::connect(server_address.as_str()).await.unwrap();

    let server_connection_arc = Arc::new(Mutex::new(server_connection));

    println!("[INFO] Connected to the server {}", server_address.as_str());

    loop {
        let server_connection_cloned = Arc::clone(&server_connection_arc);

        let service_connection = TcpStream::connect(format!("{}:{}", LOCAL_ADDR, service_port))
            .await
            .expect(
                format!(
                    "[ERROR] Cannot connect to the service at port {}",
                    service_port
                )
                .as_str(),
            );

        println!("[INFO] Connected to the service as port {}", service_port);

        let handle = tokio::spawn(async move {
            // let mut server_connection = server_connection;
            let mut service_connection = service_connection;

            let mut server_connection = server_connection_cloned.lock().await;

            let mut server_buffer: [u8; 2048] = [0; 2048];
            let mut service_buffer: [u8; 2048] = [0; 2048];
            loop {
                select! {
                    result= server_connection.read(&mut server_buffer) =>{
                    let read_size= result.unwrap();
                    if read_size==0{
                        break;
                    }
                     println!("[INFO] Data got from server of size {}",read_size);

                     println!("Data recieved:{:?}",std::str::from_utf8(&server_buffer[..read_size]));

                    service_connection.write_all(&server_buffer[..read_size]).await.unwrap();
                    },
                    result= service_connection.read(&mut service_buffer)=>{
                        let read_size= result.unwrap();
                        if read_size==0{
                           break;
                        }
                     println!("[INFO] Data got from service of size {}",read_size);
                     println!("Data recieved:{:?}",std::str::from_utf8(&service_buffer[..read_size]));


                        server_connection.write_all(&service_buffer[..read_size]).await.unwrap();
                    }
                }
            }
        });

        handle.await.unwrap();
    }
}
