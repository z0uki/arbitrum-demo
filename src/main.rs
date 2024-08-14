use futures::StreamExt;
use tokio_tungstenite::Connector;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "wss://arb1.arbitrum.io/feed".parse::<Url>()?;

    let tls = native_tls::TlsConnector::builder()
        // .add_root_certificate(cert)
        .build()
        .unwrap();

    let connector = Connector::NativeTls(tls);

    let (mut ws_stream, _response) =
        tokio_tungstenite::connect_async_tls_with_config(url, None, false, Some(connector)).await?;
    while let Some(msg) = ws_stream.next().await {
        println!("Message: {:?}", msg);
    }

    Ok(())
}
