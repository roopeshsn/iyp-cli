use clap::Parser;
use iyp_cli::{Cli, Commands};
use neo4rs::{query, Graph};
use rustyline::DefaultEditor;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

static mut GLOBAL_GRAPH: Option<Arc<Graph>> = None;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Connect(obj) => {
            if let Err(err) = connect(&obj.uri, &obj.username, &obj.password).await {
                println!("Failed to connect to Neo4j database: {:?}", err);
                return;
            }
        }
    }
}

#[derive(Debug)]
enum CustomError {
    Neo4rsError(neo4rs::Error),
    ReadlineError(rustyline::error::ReadlineError),
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CustomError::Neo4rsError(err) => Some(err),
            CustomError::ReadlineError(err) => Some(err),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Neo4rsError(err) => write!(f, "Neo4rs Error: {}", err),
            CustomError::ReadlineError(err) => write!(f, "Readline Error: {}", err),
        }
    }
}

async fn connect(
    uri: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let graph = Arc::new(Graph::new(uri, username, password).await?);
    unsafe {
        GLOBAL_GRAPH = Some(graph);
    }
    println!("Connected to Neo4j database at: {}", uri);

    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ")?;
        println!("{}", readline);
        execute_query(&readline).await;
        // match readline {
        //     Ok(line) => {
        //         println!("Line: {}", line);
        //     }
        // }
    }
}

async fn execute_query(line: &String) {
    unsafe {
        if let Some(graph) = &GLOBAL_GRAPH {
            let mut result = graph.execute(query(line)).await.unwrap();
            while let Some(row) = result.next().await.unwrap() {
                println!("{:?}", row);
            }
        } else {
            println!("You need to connect to the database first.");
        }
    }
}

async fn test_query() {
    unsafe {
        if let Some(graph) = &GLOBAL_GRAPH {
            let mut result = graph
                    .execute(
                        query("MATCH (a:AS {asn: $asn})-[:NAME]-(b:Name) RETURN a.asn as asn, b.name AS name")
                            .param("asn", 2497),
                    )
                    .await
                    .unwrap();
            while let Some(row) = result.next().await.unwrap() {
                println!("{:?}", row);
            }
        } else {
            println!("You need to connect to the database first.");
        }
    }
}

// Archived
// let uri = "iyp.iijlab.net:7687";
// let user = "neo4j";
// let pass = "neo";
// let graph = Arc::new(Graph::new(uri, user, pass).await.unwrap());
// let mut result = graph
//     .execute(
//         query("MATCH (a:AS {asn: $asn})-[:NAME]-(b:Name) RETURN a.asn as asn, b.name AS name")
//             .param("asn", 2497),
//     )
//     .await
//     .unwrap();
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
//     if let Some(name) = DefaultEditor::row.get::<String>("name") {
//         println!("Name: {}", name);
//     } else {
//         println!("Name property not found in the result.");
//     }
// }
// while let Some(row) = result.next().await.unwrap() {
//     println!("{:?}", row);
// }
// Connect() Method Error Handling
// let result = Graph::new(uri, username, password).await;
// let graph = match result {
//     Ok(graph) => Arc::new(graph),
//     Err(err) => {
//         return Err(Error::from(err));
//     }
// };
