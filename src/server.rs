use std::io::Read;
use std::net::TcpListener;
pub struct Server{
        addr: String,
}
impl Server{
        pub fn new(addr: String) -> Self{
            Self{
                addr
            }
        }

        pub fn run(self){
            println!("running on {}", self.addr);
            let listener = TcpListener::bind(&self.addr).unwrap();
            
            loop{
              match listener.accept(){
                Ok((mut stream, addr)) =>{
                  let mut buffer: [u8; 1024] = [0; 1024]; 
                  match stream.read(&mut buffer){
                    Ok(_)=>{
                      println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                    },
                    Err(e) => println!("Failed to read from connection: {}", e),
                  }
                  // println!("{:?}", buffer[0]);

                },
                Err(e) => println!("Failed to establish a connection: {}", e),
              }
              let res = listener.accept();

              if res.is_err(){
                continue;
              }
              let (stream, addr) = res.unwrap();

      }
    }
}