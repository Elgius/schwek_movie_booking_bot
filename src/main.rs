// mod telegram;
mod movie;
mod scraper;


#[tokio::main]
async fn main() {
    println!("initializing the bot");
    // telegram::telegram_caller();
    // movie::movie()
    scraper::initialised().await;
}
