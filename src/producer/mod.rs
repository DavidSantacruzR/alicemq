use rabbitmq_stream_client::{Environment, types::Message};

pub struct Publisher {
    data: String,
    event_queue: String
}

impl Publisher {
    pub fn new() -> PublisherBuilder {
        PublisherBuilder {
            data: None,
            event_queue: None
        }
    }
    pub async fn send_message(self) -> Result<(), Box<dyn std::error::Error>> {
        let event_queue = self.event_queue.as_str();
        let environment = Environment::builder().build().await?;
        let mut producer = environment.producer().name("test").build(event_queue).await?;
        producer
            .send_with_confirm(Message::builder().body(format!("{:?}", &self.data)).build())
            .await?;
        producer.close().await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct PublisherBuilder {
    data: Option<String>,
    event_queue: Option<String>,
}

impl PublisherBuilder {
    pub fn event_queue(mut self, event_queue: String) -> Self {
        self.event_queue = Some(event_queue);
        self
    }
    pub fn data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }
    pub fn build(self) -> Publisher {
        Publisher {
            event_queue: self.event_queue.unwrap(),
            data: self.data.unwrap()
        }
    }
}


