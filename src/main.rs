use std::net::TcpListener; // TcpListener：Rust標準ライブラリにある構造体

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080") // bind：「自分のPCの8080番アドレスを使います」とOSに登録する
        .expect("Failed to bind"); // 失敗したら、引数のメッセージを出して終了
    // listener：待ち受け準備ができたサーバ本体

    println!("Server is running at http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("Client connected!");
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}