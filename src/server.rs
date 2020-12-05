use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
// use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;
pub struct Server {
  addr: String,
}
impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub fn run(self) {
    println!("running on {}", self.addr);
    let listener = TcpListener::bind(&self.addr).unwrap();

    loop {
      match listener.accept() {
        Ok((mut stream, addr)) => {
          let mut buffer: [u8; 1024] = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              let response = match Request::try_from(&buffer[..]) {
                Ok(request) => {
                  dbg!(request);
                  Response::new(StatusCode::Ok, Some("<h1>hello</h1>".to_string()))
                  // write!(stream, "{}", response);
                }
                Err(e) => {
                  println!("Failed to parse a request: {}", e);
                  Response::new(StatusCode::BadRequest, None)
                },
              };
              if let Err(e) = response.send(&mut stream){
                println!("Failed to send response: {}", e);
              }
            }
            Err(e) => println!("Failed to read from connection: {}", e),
          }
          // println!("{:?}", buffer[0]);
        }
        Err(e) => println!("Failed to establish a connection: {}", e),
      }
      let res = listener.accept();

      if res.is_err() {
        continue;
      }
      let (stream, addr) = res.unwrap();
    }
  }
}
