use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
/*
sshd:22 <--> server <--> client <--> ssh-client
      upstream   downstream
                    upstream downstream
*/
async fn handle_connection(mut socket: TcpStream) {
    let prefix = format!(
        "[ {} <-> {} <-> {} ]",
        socket.peer_addr().unwrap(),
        "127.0.0.1:8888",
        "127.0.0.1:22"
    );
    println!("{} downstream connected", &prefix);

    // 连接127.0.0.1：22
    let mut upstream = TcpStream::connect("127.0.0.1:22")
        .await
        .expect(&format!("{} connect upstream failed", prefix));
    println!("{} upstream connected", prefix);

    let mut buf = [0; 16 * 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                // 连接已关闭
                println!("{} downstream disconnected", &prefix);
                return;
            }
            Ok(n) => {
                //if socket.write_all(&buf[0..n]).await.is_err() {
                //    return;
                //}
                upstream.write_all(&mut buf[0..n]).await.unwrap();
                match upstream.read(&mut buf).await {
                    Ok(0) => {
                        println!("{} upstream disconnected", &prefix);
                        return;
                    }
                    Ok(n) => {
                        socket.write_all(&buf[0..n]).await.unwrap();
                    }
                    Err(_) => {
                        eprintln!("{} upstream read error", &prefix);
                        return;
                    }
                }
            }
            Err(_) => {
                eprintln!("{} read error", &prefix);
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
