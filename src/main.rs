use openiap_client::{
    self, Client, RegisterQueueRequest
};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let b = Client::new();
    let res = b.connect_async("").await;
    match res {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to connect to server: {:?}", e);
            // return Err(e.to_string().into());
            return Ok(());
        }
    };

    let client = b.clone();
    b.on_event(Box::new(move |_event| {
        match _event {
            openiap_client::ClientEvent::Connecting => println!("CLI: Client connecting!"),
            openiap_client::ClientEvent::Connected => println!("CLI: Client connected!"),
            openiap_client::ClientEvent::Disconnected(e) => {
                println!("CLI: Client disconnected! {:?}", e)
            }
            openiap_client::ClientEvent::SignedIn => {
                println!("CLI: Client signed in!");
                // Cannot use .await in this closure. Spawn a task instead.
                let client = client.clone();
                tokio::spawn(async move {
                    let q: Result<String, openiap_client::OpenIAPError> = client
                        .register_queue(
                            RegisterQueueRequest::byqueuename("test2queue"),
                            std::sync::Arc::new(|_client, event| {
                                println!(
                                    "Received message queue from {:?} with reply to {:?}: {:?}",
                                    event.queuename, event.replyto, event.data
                                );
                                Box::pin(async { Some("{\"payload\":\"Bettina\"}".to_string()) })
                            }),
                        )
                        .await;
                    match q {
                        Ok(response) => println!("CLI: Registered queue as {:?}", response),
                        Err(e) => println!("CLI: Failed to register queue: {:?}", e),
                    }
                });
            }
            // openiap_client::ClientEvent::SignedOut => println!("CLI: Client signed out!"),
        }
    })).await;


    let mut input = String::from("bum");
    while !input.eq_ignore_ascii_case("quit") {
        input = keyboard_input().await;
    }
    Ok(())
}

pub async fn keyboard_input() -> String {
    println!("Enter your message: ");
    let mut inp = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    reader
        .read_line(&mut inp)
        .await
        .expect("Failed to read line");
    inp.trim().to_string()
}
