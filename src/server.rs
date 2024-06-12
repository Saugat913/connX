use std::sync::Arc;
use tokio::{
    // io::BufReader,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    select,
    sync::Mutex,
};

pub async fn run_as_server(client_port: String, customer_port: String) {
    const LOCAL_ADDR: &str = "0.0.0.0";

    //[SETUP] For listening the client
    let client_listener = TcpListener::bind(format!("{}:{}", LOCAL_ADDR, client_port))
        .await
        .expect("Failed to bind to address");

    let customer_listener = TcpListener::bind(format!("{}:{}", LOCAL_ADDR, customer_port))
        .await
        .expect("[ERROR] Failded to bind for customer_listener");

    println!(
        "[INFO] Server listening on {} for client",
        format!("{}:{}", LOCAL_ADDR, client_port)
    );
    println!(
        "[INFO] Server listening on {} for customer",
        format!("{}:{}", LOCAL_ADDR, customer_port)
    );

    loop {
        let (client_stream, _) = client_listener.accept().await.unwrap();
        println!(
            "[INFO] Accepted connection from: {} client",
            client_stream.peer_addr().unwrap()
        );

        let client_stream_arc = Arc::new(Mutex::new(client_stream));

        tokio::spawn(async move {


            
        });
        loop {
            let client_stream_arc_cloned = Arc::clone(&client_stream_arc);

            let (mut customer_stream, customer_addr) = customer_listener.accept().await.unwrap();
            println!(
                "[INFO] Accepted connection from: {} customer",
                customer_addr
            );

            tokio::spawn(async move {
                let mut customer_buffer: [u8; 1024] = [0; 1024];
                let mut client_buffer: [u8; 1024] = [0; 1024];
                let mut client_stream = client_stream_arc_cloned.lock().await;
                let (mut client_stream_reader, mut client_stream_writer) = client_stream.split();

                // let mut customer_buffer = String::new();
                // let mut client_buffer = String::new();

                // let customer_buff_reader = BufReader::new(customer_stream);
                // let client_buff_reader = BufReader::new(client_stream_reader);

                loop {
                    select! {
                        result= customer_stream.read(&mut customer_buffer) =>{

                            let read_size= result.unwrap();

                            if read_size==0{
                                break;
                            }
                         client_stream_writer.write_all(&customer_buffer[..read_size]).await.unwrap();
                        },
                        result= client_stream_reader.read(&mut client_buffer)=>{
                            let read_size= result.unwrap();
                            if read_size==0{
                                break;
                            }
                         customer_stream.write_all(&client_buffer[..read_size]).await.unwrap();
                        }
                    }
                }
            });
        }
    }
}
