use clap::{Parser,Subcommand, Args};
use bytes::Bytes;
use std::time::Duration;
use std::num::ParseIntError;

fn main() {
    let cli = Cli::parse();
    println!("cli, {:?}",cli);

    let addr = format!("{}:{}",cli.host, cli.port);
    println!("addr:{}",addr);
    
    match cli.command {
        Commands::Ping(Ping {msg}) => {
          println!("ping {:?}", msg)
        },
        Commands::Get(Get { key }) =>  {
          println!("get {:?}",key)
        },
        Commands::Set(Set { key, value, expires }) => {
          println!("set,key:{:?},value:{:?},expire:{:?}",key,value,expires)
        },
        Commands::Publish(Publish { channel, msg }) => {
          println!("publish channel:{:?},msg:{:?}", channel, msg)
        },
        Commands::Subscribe(Subscribe { channels }) => {
          println!("subscribe:{:?}", channels)
        },
    }

}

#[derive(Parser,Debug)]
#[command(author, name = "my-redis-cli", version, about, long_about)]
///
///
/// this is my redis client
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, id="HOSTNAME", help = "redis server host", default_value = "127.0.0.1")]
    host: String,
    /// port
    #[arg(long, short, help="redis server port",default_value_t = 4444)]
    port: u16
}


#[derive(Subcommand,Debug)]
enum  Commands {
    /// ping the server
    Ping(Ping),
    /// get key from server
    Get(Get),
    /// set key value 
    Set(Set),
    /// publish
    Publish(Publish),
    /// subscribe
    Subscribe(Subscribe)
}

#[derive(Args,Debug)]
struct Ping {
  #[arg(long)]
  msg: Option<String>
}
#[derive(Args,Debug)]
struct Get {
  #[arg(long, id="KEY")]
  key: String
}
#[derive(Args,Debug)]
struct Set {
  #[arg(long)]
  key: String,
  #[arg(long)]
  value: Bytes,
  #[arg(long,value_parser = duration_from_ms_str)]
  expires: Option<Duration>
}
#[derive(Args,Debug)]
struct Publish {
  channel: String,
  msg: Bytes
}
#[derive(Args,Debug)]
struct Subscribe {
  channels: Vec<String>
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
  let m = src.parse::<u64>()?;
  Ok(Duration::from_millis(m))
}
