// use std::error::Error;

use actix_web::{
    guard,
    web::{self, Data},
    App, HttpResponse, HttpServer, Result,
};
use async_graphql::{http::GraphiQLSource, *};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
// use async_graphql_poem::*;
// use poem::{listener::TcpListener, web::Html, Result, *};

struct Query;

// #[handler]
// async fn graphiql() -> impl IntoResponse {
//     Html(GraphiQLSource::build().endpoint("/").finish())
// }

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

type TSchema = Schema<Query, EmptyMutation, EmptySubscription>;

fn get_schema() -> TSchema {
    return Schema::build(Query, EmptyMutation, EmptySubscription).finish();
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // create the schema
//     let schema = get_schema();

//     // start the http server
//     let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
//     println!("GraphiQL: http://localhost:8000");
//     Server::new(TcpListener::bind("0.0.0.0:8000"))
//         .run(app)
//         .await
//         .unwrap();

//     Ok(())
// }

async fn index(schema: web::Data<TSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = get_schema();

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
