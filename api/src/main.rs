use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, post, web::{Json, Path}, EndpointExt, Route, Server
};

use store::Store;

pub mod request_input;
pub use request_input::*;

pub mod request_output;
pub use request_output::*;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello: {website_id}")
}

#[handler]
fn create_website(Json(data): Json<CraeteWebsiteInput> ) -> Json<CreateWebsiteOutput> {
    // poem handle the data check or throw error if there is mismathc
    let url = data.url;
    // persisit it in the DB
    let s = Store{};
    let id = s.create_website();
    let response = CreateWebsiteOutput {
        id
    };

    Json(response)
}

#[handler]
fn get_prince() -> String {
    String::from("Hello prince")
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let app = Route::new().at("/website/:website_id", get(get_website))
                                 .at("/website", post(create_website))
                                 .at("/prince", get(get_prince));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}