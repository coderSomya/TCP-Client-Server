use clap::{Parser, Subcommand};

pub mod connect;
pub mod stream;

#[derive(Parser)]
#[command(name = "command")]
#[command(about = "A CLI command", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Connect {
        #[arg(long)]
        client_host: String,
        #[arg(long)]
        client_port: u16,
    },
    Serve {
        #[arg(long)]
        server_host: String,
        #[arg(long)]
        server_port: u16,
    },
}


async fn run()-> Result<(), String>{

    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();

    tokio::spawn(
        async move {
            tokio::io::copy(&mut stdin, &mut stdout).await.unwrap();    
        }
    ).await.unwrap();
    
    Ok(())
}

fn main(){
    println!("hello from rust networking...");

    // run().await.unwrap();
    // stream::client().await;


    let cli = Cli::parse();

    let runtime = tokio::runtime::Runtime::new().unwrap();

    match &cli.command{
        Commands::Connect{client_host, client_port} =>{

            runtime.block_on(async move{

                tokio::select! {
                    _ = stream::client()=> {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            });
        },
        Commands::Serve {server_host,  server_port }=>{
            runtime.block_on(async move{
                
                tokio::select! {
                    _ = stream::server()=> {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            });        
        }
    }
}