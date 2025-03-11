pub mod pistachio_server {

  use std::{
    env,
    io::{prelude::*, Error, Read, BufReader},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    fs
  };

  pub struct PistachioServer {
    host_address: Ipv4Addr,
    port: u16,
  }
  
  pub fn create() -> Result<PistachioServer, Error> {
    let webserver_instance = PistachioServer {
      host_address: Ipv4Addr::new(127, 0, 0, 1),
      port: 8080
    };
    Ok(webserver_instance)
  }
  
  impl PistachioServer {
    pub fn run(&self) {
      println!("Listening on {}:{}", self.host_address, self.port);
      let listener = TcpListener::bind(
        SocketAddrV4::new(self.host_address, self.port)
      ).expect("Error: cannot start listening for connections");

      for stream in listener.incoming() {
        println!("Received connection");
        let connection = stream.expect(
          "Cannot establish connection"
        );
        self.handle_request(connection);
      }
    }

    fn handle_request(&self, mut connection: TcpStream) {
      let buf_reader = BufReader::new(&connection);
      let http_request: Vec<_> = buf_reader
          .lines()
          .map(|result| result.unwrap())
          .take_while(|line| !line.is_empty())
          .collect();

      self.send_response(connection);
    }

    fn send_response(&self, mut connection: TcpStream) {
      // CRLF -> \r\n\r\n
      let status_line = "HTTP/1.1 200 OK";

      // cconnection.write_all(response.as_bytes()).unwrap();
      let contents = fs::read_to_string("src\\resources\\hello.html").unwrap();
      let length = contents.len();
  
      let response =
          format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
  
      connection.write_all(response.as_bytes()).unwrap();
    }
  
/*     fn project_dir(&self) -> Result<String, Error> {
      let dir = env::current_dir()?;
      let x = format!("{}", dir.display());
      Ok(x)
    } */
    
  /*   pub fn debug(&self) -> Result<> {
      let mut file_path = self.project_dir()?.clone();
      file_path.push_str("\\conf\\settings.conf");
    
      let mut file = File::open(file_path)?;
      let mut content_vector: Vec<u8> = vec![];
      file.read_to_end(&mut content_vector)?;
      let contents = format!("{}", content_vector.bytes().collect());
    
    } */
  }
}
