use std::net::TcpListener; // TcpListener：Rust標準ライブラリにある構造体
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080") // bind：「自分のPCの8080番アドレスを使います」とOSに登録する
        .expect("Failed to bind"); // 失敗したら、引数のメッセージを出して終了
    // listener：待ち受け準備ができたサーバ本体

    println!("Server is running at http://127.0.0.1:8080");

    // 接続が来るたびに TcpStream を受け取る
    for stream in listener.incoming() { // listener.incoming()：OSの待ち受けキュー
        match stream {
            Ok(mut stream) => {
                println!("Client connected!");

                // クライアントから送られてきたデータを読み込むためのバッファ
                let mut buffer = [0; 1024];

                // TcpStreamから最大1024バイト読み込む
                let n = stream // n：何バイト読めたか(readの返り値)
                    .read(&mut buffer)
                    .expect("Failed to read from stream");

                println!("----- HTTP Request -----");
                // HTTPはネットワーク上では文字ではなくバイト列として送られてくるので、文字列に変換
                println!("{}", String::from_utf8_lossy(&buffer[..n]));  // 読み込めた分だけ出力
                println!("------------------------");
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}