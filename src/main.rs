/// Supported Server Modules
mod httpdir;

use structopt::StructOpt;
use tokio::io;

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "server mode")]
pub enum ServerMode {
    HTTPDir(httpdir::Args),
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "servar", about = "Univsersal Server Tool")]
pub struct GlobalArgs {
    #[structopt(short, long, help = "Listen Port", default_value = "8000")]
    pub port: u16,
    #[structopt(short, long, help = "Listen IP", default_value = "0.0.0.0")]
    pub ip: String,
    #[structopt(about = "Mode", subcommand)]
    pub mode: ServerMode,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = GlobalArgs::from_args();

    let gargs = args.clone();

    // Add new Server Module execs here
    match args.mode {
        ServerMode::HTTPDir(mod_args) => httpdir::exec(gargs, mod_args).await?,
    }

    return Ok(());
}
