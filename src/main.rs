use mongodb::{
    bson::{Bson, Document},
    options::ClientOptions,
    Client,
};
use serde_json::Value;
use std::{convert::TryInto, env, io::Read};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input = match args.get(1) {
        Some(input) => input.clone(),
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    };

    if raw_input.is_empty() {
        panic!("No input found. Use either first cli arg or stdin.");
    }

    let json: Value = serde_json::from_str(raw_input.as_str()).expect("Invalid json");

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(
        "mongodb://prisma:prisma@127.0.0.1:27017/testdb?authSource=admin&retryWrites=true",
    )
    .await
    .unwrap();

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    let bson: Bson = json.try_into().unwrap();
    let cmd: Document = if let Bson::Document(document) = bson {
        document
    } else {
        panic!("Bson is not a document");
    };

    let cmd_result = client
        .database("test")
        .run_command(cmd, None)
        .await
        .unwrap();

    println!("{}", cmd_result);
}
