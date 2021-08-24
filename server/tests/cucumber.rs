use cucumber_rust::{async_trait, Context, Cucumber, World};
use rocket::local::asynchronous::Client;
use std::convert::Infallible;



mod steps;

pub enum MyWorld {
    Nothing,
    TextResponse(String),
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Nothing)
    }
}

#[tokio::main]
async fn main() {
    let rocket = server::config();
    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket instance");

    Cucumber::<MyWorld>::new()
        .features(&["./features"])
        .steps(steps::index::steps())
        .context(Context::new().add(client))
        .cli()
        .run_and_exit()
        .await
}
