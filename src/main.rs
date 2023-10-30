pub mod assets;
mod entsoe;
use axum::{
    routing::{self, get},
    Router, Server,
};
use hyper::Error;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod forecast {
    use crate::assets::AREA_CODE;
    use crate::entsoe::day_ahead_load;
    use axum::extract::Path;

    pub async fn forecast(Path(area_code): Path<AREA_CODE>) -> String {
        print!("   Area Code: {:?}", area_code);
        day_ahead_load(area_code).await;
        "Hello".to_string()
    }
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            hello::world,
        ),
        components(
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "todo", description = "Todo items management API")
        )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
                )
            }
        }
    }

    // build our application with a single route
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/forecast/:area_code", get(forecast::forecast));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod hello {
    use axum::Json;

    /// List all Todo items
    ///
    /// List all Todo items from in-memory storage.
    #[utoipa::path(
        get,
        path = "/world",
        responses(
            (status = 200, description = "List all todos successfully", body = [Todo])
        )
    )]
    pub(super) async fn world() -> Json<Vec<String>> {
        let return_string = "Hello, World!".to_owned();
        Json(vec![return_string])
    }
}
