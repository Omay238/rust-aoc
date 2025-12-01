use rustls;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

pub fn get_input(year: String, day: String, session: Option<String>) -> String {
    let session = session.unwrap_or(std::env::var("SESSION").unwrap());

    let mut tcp = TcpStream::connect("adventofcode.com:443").unwrap();
    tcp.set_nonblocking(false).unwrap();

    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = Arc::new(
        rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth(),
    );

    let url = "adventofcode.com".try_into().unwrap();
    let mut tls = rustls::ClientConnection::new(config, url).unwrap();

    let mut tls_stream = rustls::Stream::new(&mut tls, &mut tcp);

    let req = format!(
        "GET /{year}/day/{day}/input HTTP/1.1\r\n\
         Host: adventofcode.com\r\n\
         User-Agent: aoc_lib/0.1\r\n\
         Cookie: session={session}\r\n\
         Connection: close\r\n\
         \r\n"
    );

    tls_stream.write_all(req.as_bytes()).unwrap();
    tls_stream.flush().unwrap();

    let mut response = String::new();
    tls_stream.read_to_string(&mut response).unwrap();

    response
}
