use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError, QueueProperties};
use std::{thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        thread::sleep(time::Duration::from_millis(1000));
        println!("Di Komputer 2406495703. Pesan diterima: {:?}", message);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let subscriber = CrosstownBus::new_subscriber("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    _ = subscriber.subscribe("user_created".to_owned(), UserCreatedHandler, QueueProperties {
        auto_delete: false,
        durable: false,
        use_dead_letter: true,
        consume_queue_name: None,
    });
    
    loop {
        tokio::time::sleep(time::Duration::from_millis(1000)).await;
    }
}
