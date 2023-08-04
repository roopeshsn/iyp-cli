use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Connect with the DB
    Connect(Connect),

    /// Run query
    Query(Query),
}

#[derive(Debug, Args)]
pub struct Connect {
    pub uri: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Args)]
pub struct Query {
    pub query: String,
}
