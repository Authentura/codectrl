use codectrl_server::run_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_server(None, None, None, None).await?;

    Ok(())
}
