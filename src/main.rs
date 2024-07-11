mod client;
mod server;
mod common;

use structopt::StructOpt;
use common::logger::init_logger;
use common::config::Settings;

#[derive(StructOpt)]
#[structopt(name = "chat", about = "A simple chat application.")]
enum ChatOption {
    #[structopt(about = "Run as server")]
    Server,
    #[structopt(about = "Run as client")]
    Client,
}

#[tokio::main]
async fn main() {
    let settings = Settings::new("config.toml");
    init_logger(settings.log.level);

    let chat_option = ChatOption::from_args();
    match chat_option {
        ChatOption::Server => {
            let err = server::run_server(&settings.server).await;
            if let Err(e) = err {
                log::error!("Error running server: {}", e);
            }

        }
        ChatOption::Client => {
            let err = client::run_client(&settings.server);
            if let Err(e) = err {
                log::error!("Error running client: {}", e);
            }
        }
    }
}
