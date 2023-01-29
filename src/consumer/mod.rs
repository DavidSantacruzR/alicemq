use tokio::{task};
use futures::StreamExt;
use tracing::{info, Level};
use rabbitmq_stream_client::{Environment};
use rabbitmq_stream_client::error::{StreamCreateError};
use rabbitmq_stream_client::types::OffsetSpecification;
use tracing_subscriber::FmtSubscriber;

pub struct Callback {
    event_queue : String,
    environment: Environment,
}

impl Callback {
    pub fn new() -> CallbackBuilder {
        CallbackBuilder::default()
    }
    pub async fn handle(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        let event_queue = self.event_queue.clone();
        let mut consumer = self.environment.consumer().offset(OffsetSpecification::First).build(event_queue.as_str()).await?;
        task::spawn( async move {
            while let Some(delivery) = consumer.next().await {
                info!("Got message {:?}", delivery)
            }
        });
        Ok(())
    }
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
    pub async fn callback(self) -> Result<Callback, Box<dyn std::error::Error>> {
        Ok(Callback {
            event_queue: self.event_queue.ok_or("Not possible to create event queue.")?,
            environment: self.environment.ok_or("Not possible to fetch environment")?
        })
    }
}
