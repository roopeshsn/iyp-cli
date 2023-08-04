use clap::Parser;
use iyp_cli::{Cli, Commands};
use neo4rs::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // println!("{:?}", args);

    match &args.command {
        Commands::Connect(obj) => {
            println!("{:?}", obj.uri);
            print!("From command")
        }
        Commands::Query(obj) => {
            println!("{:?}", obj);
        }
    }

    let uri = "iyp.iijlab.net:7687";
    let user = "neo4j";
    let pass = "neo";

    let graph = Arc::new(Graph::new(uri, user, pass).await.unwrap());

    let mut result = graph
        .execute(
            query("MATCH (a:AS {asn: $asn})-[:NAME]-(b:Name) RETURN a.asn as asn, b.name AS name")
                .param("asn", 2497),
        )
        .await
        .unwrap();

    // Without using Some()
    // let row = result.next().await.unwrap().unwrap();
    // println!("{:?}", row.get::<String>("name").unwrap());

    // Works for one item
    // let get_result = result.next().await.unwrap();
    // if let Some(row) = get_result {
    //     if let Some(name) = row.get::<String>("name") {
    //         println!("Name: {}", name);
    //     } else {
    //         println!("Name property not found in the result.");
    //     }
    // } else {
    //     println!("No result returned from the query.");
    // }

    // while let Some(row) = result.next().await.unwrap() {
    //     if let Some(name) = row.get::<String>("name") {
    //         println!("Name: {}", name);
    //     } else {
    //         println!("Name property not found in the result.");
    //     }
    // }

    while let Some(row) = result.next().await.unwrap() {
        println!("{:?}", row);
    }
}
