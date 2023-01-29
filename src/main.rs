mod consumer;
mod producer;
use crate::consumer::{Callback};
use crate::producer::{Publisher};

#[tokio::main]
async fn main() -> Result<(), Box< dyn std::error::Error>> {
    let mut new_consumer = Callback::new()
        .queue("test")
        .await?
        .callback()
        .await?;
    new_consumer.handle().await?;

    let publisher = Publisher::new()
        .event_queue("test".to_string())
        .data("Some data".to_string())
        .build();
    publisher.send_message().await?;
    Ok(())
}
