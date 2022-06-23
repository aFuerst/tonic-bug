extern crate pinger_lib;

use std::time::SystemTime;

use pinger_lib::greeter_client::GreeterClient;
use pinger_lib::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut handles = Vec::new();

  for _ in 0..2 {
    handles.push(tokio::spawn( async move {
      let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

      let request = tonic::Request::new(HelloRequest {
          name: "Tonic".into(),
      });
  
      let start = SystemTime::now();
      let _response = client.say_hello(request).await.unwrap();
      let elapsed_sec = start.elapsed().unwrap().as_millis() as u64 as f64 / 1000.0;
        
      println!("RESPONSE took {} sec", elapsed_sec);
    }));
  }

  for handle in handles {
    handle.await?;
  }


  Ok(())
}