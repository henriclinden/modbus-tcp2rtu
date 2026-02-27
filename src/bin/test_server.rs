use std::future;
use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Service};

struct Slave;

impl Service for Slave {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadCoils(addr, count) => {
                println!("Received ReadCoils: addr={}, count={}", addr, count);
                let mut responses = vec![false; count as usize];
                if !responses.is_empty() {
                    responses[0] = true;
                }
                future::ready(Ok(Response::ReadCoils(responses)))
            }
            _ => {
                println!("Received unimplemented request: {:?}", req);
                // Return the specific code for an illegal function
                future::ready(Err(ExceptionCode::IllegalFunction))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = tokio_serial::new("/dev/pts/2", 9600);
    let server_serial = tokio_serial::SerialStream::open(&builder)?;

    println!("Starting RTU Slave on /dev/pts/2...");
    let server = server::rtu::Server::new(server_serial);
    server.serve_forever(&Slave).await?;

    Ok(())
}
