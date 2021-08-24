use crate::MyWorld;
use cucumber_rust::{t, Steps};
use rocket::http::Status;
use rocket::local::asynchronous::Client;

pub fn steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.when_async(
        "I visit the homepage",
        t!(|_world, ctx| {
            let client = ctx.get::<Client>().unwrap();
            let response = client.get("/").dispatch().await;
            assert_eq!(response.status(), Status::Ok);
            let str_response = response.into_string().await;
            assert!(str_response.is_some());
            MyWorld::TextResponse(str_response.unwrap())
        }),
    );

    steps.then_regex(r#"I should see some text saying "(.*)"$"#, |world, ctx| {
        match world {
            MyWorld::TextResponse(response) => {
                assert_eq!(response, ctx.matches[1]);
            }
            _ => unreachable!(),
        }
        MyWorld::Nothing
    });

    steps
}
