mod consumer;
mod loan_handlers;
use crate::consumer::{Callback, ConsumerHandler};

#[tokio::main]
async fn main() -> Result<(), Box< dyn std::error::Error>> {
    let new_handler = ConsumerHandler::default();
    let new_consumer = Callback::new()
        .queue("loan_request")
        .await?
        .consume(new_handler)
        .await;
    Ok(())
}
