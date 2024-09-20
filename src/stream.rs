pub async fn client() -> Result<(), String>{
    let client = tokio::net::TcpStream::connect("localhost:6969")
        .await
        .map_err(|_|{
         "could not connect to localhost"
    })?;

    let (mut reader, mut writer) = client.into_split();

    let client_read = tokio::spawn(
        async move{
            tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await
        }
    );

    let client_write = tokio::spawn(
        async move{
            tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await
        }
    );


    tokio::select!{
        _ = client_read=> {},
        _ = client_write => {}
    }

    Ok(())
}

pub async fn server() -> Result<(), String>{
    let server = tokio::net::TcpListener::bind("localhost:6969")
        .await
        .map_err(|_|{
         "could not bind to localhost"
    })?;


    let (handle, _) = server.accept().await.map_err(|_e| {
        "could not accept connection "
    })?;

    let (mut reader, mut writer) = handle.into_split();

    let client_read = tokio::spawn(
        async move{
            tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await
        }
    );

    let client_write = tokio::spawn(
        async move{
            tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await
        }
    );


    tokio::select!{
        _ = client_read=> {},
        _ = client_write => {}
    }

    Ok(())
}