use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tracing::info;

static ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let listener = TcpListener::bind(ADDR)
        .await
        .with_context(|| format!("binding tcp listener to addr: {ADDR}"))?;

    info!("server listening on addr: {ADDR}");

    loop {
        let (socket, _) = listener.accept().await.context("accepting connection")?;
        todo!()
        // process_socket(socket).await;
    }

    Ok(())
}
