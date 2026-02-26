use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_modbus::prelude::*;
use tokio_modbus::server::{tcp::Server, Service};
use tokio_serial::SerialStream;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(version, about = "Modbus TCP to RTU Bridge")]
struct Args {
    /// TCP address to bind the Modbus TCP server to
    #[arg(short, long, default_value = "0.0.0.0:5020")]
    tcp_bind: SocketAddr,

    /// Serial port device path
    #[arg(short, long, default_value = "/dev/ttyUSB0")]
    serial_port: String,

    /// Baud rate for the serial port
    #[arg(short, long, default_value_t = 9600)]
    baud_rate: u32,
}

#[derive(Clone)]
struct BridgeService {
    // We share a single RTU client instance among all TCP client tasks
    client: Arc<tokio::sync::Mutex<tokio_modbus::client::Context>>,
}

impl Service for BridgeService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Self::Response, Self::Exception>>
                + Send,
        >,
    >;

    fn call(&self, req: Self::Request) -> Self::Future {
        let client = self.client.clone();
        Box::pin(async move {
            let mut client = client.lock().await;
            match client.call(req).await {
                Ok(Ok(resp)) => Ok(resp),
                Ok(Err(err)) => Err(err),
                Err(err) => {
                    error!("Modbus RTU request failed: {}", err);
                    Err(ExceptionCode::ServerDeviceFailure)
                }
            }
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    info!("Starting Modbus TCP to RTU Bridge");
    info!("TCP Bind: {}", args.tcp_bind);
    info!("Serial Port: {}", args.serial_port);
    info!("Baud Rate: {}", args.baud_rate);

    // Open UART device
    let builder = tokio_serial::new(&args.serial_port, args.baud_rate);
    let port = SerialStream::open(&builder)?;

    // Create Modbus RTU client context
    let rtu_client = tokio_modbus::client::rtu::attach(port);
    // Convert to shared Context
    let shared_client = Arc::new(tokio::sync::Mutex::new(rtu_client));

    let listener = TcpListener::bind(args.tcp_bind).await?;
    info!("Modbus TCP Server listening on {}", args.tcp_bind);

    let server = Server::new(listener);

    let new_service = move |socket_addr| {
        info!("Accepted TCP connection from {}", socket_addr);
        Ok(Some(BridgeService {
            client: shared_client.clone(),
        }))
    };

    let on_connected = |stream, socket_addr| {
        let new_service = new_service.clone();
        async move {
            tokio_modbus::server::tcp::accept_tcp_connection(stream, socket_addr, new_service)
        }
    };
    let on_process_error = |err| {
        error!("Modbus server error: {}", err);
    };

    server.serve(&on_connected, on_process_error).await?;

    Ok(())
}
