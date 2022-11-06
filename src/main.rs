use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use futures::future::join_all;

use std::error::Error;
use std::time::{Instant};

const REQ_TO_MAKE: usize = 100;

// Makes a connection, writes a message, then reads a message and exits
async fn make_req() {
  let mut stream = TcpStream::connect("127.0.0.1:8080")
    .await
    .unwrap_or_else(|_| {
      panic!("Error Connecting to stream");
    });

  let _result = stream.write_all(b"hello world\n").await;
  stream.read(&mut [0; 128]).await.unwrap_or_else(|_| {
    0
  });
  stream.flush().await.expect("flushing failed");
  stream.shutdown().await.expect("shutdown failed");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
  let start_time = Instant::now();
  let mut handles = Vec::with_capacity(REQ_TO_MAKE);
  for _ in 1..REQ_TO_MAKE {
    let handle = tokio::spawn(async move {
        make_req().await
    });
    handles.push(handle);
  }
  join_all(handles).await;

  println!("Asynchronous Total Time = {:?}", start_time.elapsed());

  //synchronous
  for _ in 1..REQ_TO_MAKE {
    make_req().await;
  }

  println!("Synchronous Total Time = {:?}", start_time.elapsed());
  Ok(())
}