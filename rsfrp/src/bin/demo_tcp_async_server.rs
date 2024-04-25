use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut socket: TcpStream) {
    let log_prefix = format!("[ {} ]", socket.peer_addr().unwrap());

    println!("{} connected", log_prefix);

    let mut buf = [0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                // 连接已关闭
                println!("{} disconnected", log_prefix);
                return;
            }
            Ok(n) => {
                println!(
                    "{} recv: {:?}",
                    log_prefix,
                    std::str::from_utf8(&buf[0..n]).unwrap()
                );
                buf.iter_mut().for_each(|x| *x += 1);
                if socket.write_all(&buf[0..n]).await.is_err() {
                    return;
                }
            }
            Err(_) => {
                eprintln!("{} read error", log_prefix);
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:8888";
    let listener = TcpListener::bind(server_addr)
        .await
        .expect(&format!("[{}] failed to bind", server_addr));
    println!("server started.");
    println!("listening on: {}...\n\n", server_addr);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(socket));
    }
}
