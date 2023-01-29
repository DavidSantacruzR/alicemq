# AliceMQ
## Rust adapter for the official rabbitmq-stream-client

This is a personal project related to testing, the current rabbitMQ stream client official library.

The idea of this small project is to build a simple Smart Publisher / Consumer.
To create a new consumer, you can simply import the consumer, and create a new stream the following way:


```rust
let mut new_consumer = Callback::new()
    .queue("test")
    .await?
    .callback()
    .await?;
new_consumer.handle().await?;
```
To create a publisher/producer, just import the producer module. Here's how to create a publisher.

````rust
let publisher = Publisher::new()
    .event_queue("test".to_string())
    .data("Some data".to_string())
    .build();
publisher.send_message().await?;
````
Here's a full example of how to create a stream, with a specific handler, and send messages to that queue:

````rust
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
````