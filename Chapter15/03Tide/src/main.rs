use tide::Request;

async fn hello_world(_: Request<()>) -> tide::Result {
    Ok(String::from("Hello World!").into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/hello_world").get(hello_world);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
