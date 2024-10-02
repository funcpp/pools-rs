mod api;
mod pg;

use actix_cors::Cors;
use actix_web::{guard, http::header, middleware, web, App, HttpResponse, HttpServer, Result};
use api::{GQLSchema, Mutation, Query};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type EntityId = i32;
pub type DateTime = chrono::DateTime<chrono::Utc>;

async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let schema: GQLSchema =
            Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                .data(pg::establish_db_pool())
                .finish();

        App::new()
            .app_data(web::Data::new(schema.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema)),
            )
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
