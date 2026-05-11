use borsh_derive::{BorshDeserialize, BorshSerialize};
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
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
        println!("In YOUR_NPM Computer. Message received: {:?}", message);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let amqp_url = std::env::var("AMQP_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_owned());
    let subscriber = CrosstownBus::new_subscriber(amqp_url)?;
    _ = subscriber.subscribe(
        "user_created".to_owned(),
        UserCreatedHandler{},
        QueueProperties { auto_delete: false, durable: false, use_dead_letter: true, consume_queue_name: None },
    )?;
    loop {}
}