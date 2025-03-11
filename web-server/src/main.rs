use std::io::Error;
use pistachio::pistachio_server;

mod pistachio;

fn main() -> Result<(), Error>{
    let server = pistachio_server::create()?;
    server.run();
    Ok(())
}
