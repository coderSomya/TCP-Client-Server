
pub mod connect;

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

#[tokio::main]
async fn main(){
    println!("hello from networking...");

    run().await.unwrap();
}