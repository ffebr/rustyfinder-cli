mod args;

use args::Args;
use clap::Parser;
use local_ip_address::local_ip;
use std::{fs::File, io::Read};
use tiny_http::{Header, Response, Server};

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    args.validate().unwrap();

    let port = args.port.unwrap_or(0);
    let ip = local_ip().unwrap().to_string();
    let addr = format!("{}:{}", ip, port);

    let server = Server::http(&addr).unwrap();
    let actual_addr = server.server_addr();

    let file_name = args.path.file_name().unwrap().to_string_lossy().to_string();

    println!("Сервер запущен на {}", actual_addr);
    let url = format!("http::/{}/", &actual_addr.to_string());
    qr2term::print_qr(url).unwrap();

    for request in server.incoming_requests() {
        if request.url() == "/" {
            let mut file = File::open(&args.path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let response = Response::from_data(buffer).with_header(
                Header::from_bytes(
                    &b"Content-Disposition"[..],
                    format!("attachment; filename=\"{}\"", file_name),
                )
                .unwrap(),
            );
            request.respond(response)?;
        } else {
            let response = Response::from_string("404 Not Found").with_status_code(404);
            request.respond(response)?;
        }
    }

    Ok(())
}
