use codectrl_server::redirect_handler::RedirectHandler;
use log::info;
use tokio::{
    runtime::Handle,
    time::{sleep, Duration},
};

#[tokio::test]
async fn test_flow() {
    dotenv::from_filename(".env-tests").ok();
    env_logger::init();

    let redirect_handler = RedirectHandler::new(8080);

    redirect_handler.start(Handle::current());

    sleep(Duration::new(2, 0)).await;

    assert!(redirect_handler.is_started());

    redirect_handler.register();
    redirect_handler.register();
    redirect_handler.register();
    redirect_handler.register();

    assert!(redirect_handler.is_started());

    redirect_handler.unregister();
    redirect_handler.unregister();
    redirect_handler.unregister();
    redirect_handler.unregister();

    info!(target: "redirect_handler test", "Waiting 10 seconds for the handler to close...");
    sleep(Duration::new(10, 0)).await;

    assert!(!redirect_handler.is_started());
}
