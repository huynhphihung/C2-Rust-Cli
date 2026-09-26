use std::io::Result;

use crate::{
    agent::Agent,
    client::Client,
    config::Config,
    handler::{task_execute, task_handler},
    protocol::{HeartBeatMessage, RequestTaskMessage},
    types::message::Message,
};

mod agent;
mod client;
mod config;
mod handler;
mod protocol;
mod shell;
mod task;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    // load config
    let config = Config::new();

    println!("Config: {:?}", config);

    let mut heartbeat_interval = tokio::time::interval(config.heartbeat_interval);
    let mut task_interval = tokio::time::interval(config.task_delay);

    // Initialize new client
    let mut client = Client::new(config.server_addr);
    let mut agent = Agent::new(config.server_addr);
    let agent_id = agent.info.agent_id;

    client.connect_to_server().await?;

    let register_message = Message::Register(agent.info.clone());

    client.send(&register_message).await?;

    loop {
        tokio::select! {
            _ = heartbeat_interval.tick() => {
                let heartbeat = HeartBeatMessage { agent_id };
                let heartbeat_message = Message::Heartbeat(heartbeat);
                client.send(&heartbeat_message).await?;
            }

            _ = task_interval.tick() => {
                let request_task = RequestTaskMessage { agent_id };
                let request_task_message = Message::RequestTask(request_task);
                println!("RequestTaskMessage: {:?}", request_task_message);
                client.send(&request_task_message).await?;

                let response = client.receive_msg().await?;

                match response {
                    Message::Task(task) => {
                        println!("Receive task: {task:?}");
                        if let Some(writer) = client.writer.as_mut() {
                            task_handler(writer, task, &mut agent).await?;
                        }
                    }
                    Message::NoTask => {
                        println!("No task available");
                    }

                    other => {
                        println!("Unexpected message from server: {other:?}")
                    }
                }
                    }
        }
    }
}
