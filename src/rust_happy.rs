use std::{env, error::Error};

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::LevelFilter;
use log::{self, error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::bail;
use simple_logger::SimpleLogger;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Event {
    #[serde(default)]
    first_name: String,
    #[serde(default)]
    last_name: String,
}

// #[derive(Deserialize, Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
// #[serde(tag = "type")]
// num Event {
//    Name {
//        #[serde(default)]
//        first_name: String,
//        #[serde(default)]
//        last_name: String,
//    },
//}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    lambda!(my_handler);

    Ok(())
}

/// just pull the serialization into the lambda layer so we can decide how to deal with rouges.
fn my_handler(
    event: serde_json::Value,
    context: Context,
) -> Result<serde_json::Value, HandlerError> {
    log::info!("event: {}", serde_json::json!(event));

    let result: Result<serde_json::Value, HandlerError>;

    let event: std::result::Result<Event, serde_json::Error> = serde_json::from_value(event);

    let db_table_name = env::var("TABLE").unwrap();
    log::info!("moo: {} ", db_table_name);

    // TODO: move this to lazy_static
    // let s3 : S3Client = S3Client::new(Region::default());
    let region = env::var("AWS_REGION").unwrap();
    log::info!("region: {}", region);
    let _region: Region = region.parse().unwrap();
    // let _s3 : S3Client = S3Client::new(_region);

    //match s3.list_buckets().await {
    //    Err(e) => log::info!("Error listing buckets: {}", e),
    //    Ok(buckets) => log::info!("Buckets found: {:?}", buckets),
    //};
    //
    match event {
        // a message we specifically are meant to handle
        Ok(ev) => {
            let value = serde_json::to_value(ev).unwrap();
            result = Ok(value);
        }
        // some error dealing with Value -> Event serialization issue of some sort.
        Err(e) => {
            result = Err(HandlerError::from(e));
        }
    }

    result
}
