use agql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql as agql;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use model::{mutation::Mutation, query::QueryRoot, RewekeySchema};
use rocket::{http::Method, response::content, State};
use sqlx::postgres::PgPoolOptions;

use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

mod db;
mod model;

#[rocket::get("/")]
fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<RewekeySchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &State<RewekeySchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema).await
}

#[rocket::launch]
async fn rocket() -> _ {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres:///rewekey_dev")
        .await
        .unwrap();
    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(pool)
        .finish();
    let cors = cors_options().to_cors().unwrap();
    rocket::build()
        .manage(schema)
        .mount(
            "/",
            rocket::routes![graphql_query, graphql_request, graphql_playground],
        )
        .attach(cors)
}

fn cors_options() -> CorsOptions {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(Into::into)
            .collect(),
        allow_credentials: true,
        ..Default::default()
    }
}
