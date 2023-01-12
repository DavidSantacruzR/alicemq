use tokio::task;
use std::fmt;
use rabbitmq_stream_client::{Environment};
use rabbitmq_stream_client::error::{StreamCreateError};

pub struct Callback {
    event_queue : String,
    environment: Environment,
}

impl Callback {
    pub fn new() -> CallbackBuilder {
        CallbackBuilder::default()
    }
    pub async fn handle(mut self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("Message handled"))
    }
}

#[derive(Debug, Default)]
pub struct ConsumerHandler {
    data: String
}

#[derive(Default)]
pub struct CallbackBuilder {
    event_queue : Option<String>,
    environment: Option<Environment>,
}

impl CallbackBuilder {
    pub async fn queue(mut self, event_queue: impl Into<String>) -> Result<Self, StreamCreateError> {
        let event = event_queue.into();
        let environment = Environment::builder()
            .host("localhost")
            .port(5552)
            .build()
            .await?;
        match environment.delete_stream(&event).await {
            Ok(n) => format!("Successfully deleted stream {:?}", n),
            Err(e) => format!("Unable to delete stream {:?}", e)
        };
        environment
            .stream_creator()
            .create(&event)
            .await?;
        self.event_queue.get_or_insert(event);
        self.environment.get_or_insert(environment);
        Ok(self)
    }
    pub async fn consume(self, consumer_handler: ConsumerHandler) -> Callback {
        println!("{:?}", consumer_handler.data);
        Callback {
            event_queue: self.event_queue.unwrap(),
            environment: self.environment.unwrap()
        }
    }
}
