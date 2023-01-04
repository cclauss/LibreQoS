use lqos_bus::{BusRequest, BusResponse, BUS_BIND_ADDRESS, BusSession, encode_request, decode_response};
use anyhow::Result;
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

pub fn run_query(requests: Vec<BusRequest>) -> Result<Vec<BusResponse>> {
    let mut replies = Vec::new();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mut stream = TcpStream::connect(BUS_BIND_ADDRESS).await?;
            let test = BusSession {
                auth_cookie: 1234,
                requests: requests,
            };
            let msg = encode_request(&test)?;
            stream.write(&msg).await?;
            let mut buf = Vec::new();
            let _ = stream.read_to_end(&mut buf).await?;
            let reply = decode_response(&buf)?;
            replies.extend_from_slice(&reply.responses);
            Ok(replies)
        })
}