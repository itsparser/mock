use serde::{Deserialize, Serialize};
use worker::*;
use chrono::prelude::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {

    // Create an instance of the Router, which can use parameters (/user/:name) or wildcard values
    // (/file/*pathname). Alternatively, use `Router::with_data(D)` and pass in arbitrary data for
    // routes to access and share using the `ctx.data()` method.
    let router = Router::new();

    #[derive(Deserialize,Serialize)]
    enum FieldType {
        Text,
        Number,
        Boolean
    }

    #[derive(Deserialize,Serialize)]
    struct FieldSchema {
        id: String,
        #[serde(rename="type")]
        type_kind: FieldType
    }

    // useful for JSON APIs
    #[derive(Deserialize, Serialize)]
    struct FormSchema {
        id: String,
        fields: Vec<FieldSchema>
    }
    router
        .get_async("/ping", |_req, _ctx| async move {Response::ok("pong")})
        .get_async("/account/:id", |_req, ctx| async move {
            Response::error("Bad Request", 400)
        })
        .post_async("/form", |mut req: Request, ctx| async move {
            let mut _body = req.json::<Vec<FieldSchema>>().await?;
            let now: DateTime<Utc> = Utc::now();
            let form = FormSchema {
                id: now.to_string(),
                fields: _body,
            };

            Response::from_json(&form)
        })
        .post_async("/form/:id/submit", |mut req: Request, ctx| async move {
            Response::ok("pong")
        })
        .run(req, env).await
}