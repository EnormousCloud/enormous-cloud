pub mod args;
pub mod homepage;
pub mod telemetry;

#[derive(Clone)]
pub struct State {
    pub db_pool: sqlx::Pool<sqlx::postgres::Postgres>,
    pub static_dir: String,
}

use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io};
use tide::{Body, Middleware, Next, Request, Response, StatusCode};

// This is an example of middleware that keeps its own state and could
// be provided as a third party crate
#[derive(Default)]
struct ServeMiddleware {}

#[tide::utils::async_trait]
impl Middleware<State> for ServeMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        let path = req.url().path().to_owned();
        let method = req.method().to_string();

        if method == "GET" && path != "/" {
            let dir = PathBuf::from(req.state().static_dir.clone());
            let path = path.trim_start_matches('/');
            let mut file_path = dir.clone();
            for p in Path::new(path) {
                if p == OsStr::new(".") {
                    continue;
                } else if p == OsStr::new("..") {
                    file_path.pop();
                } else {
                    file_path.push(&p);
                }
            }
            let file_path = async_std::path::PathBuf::from(file_path);
            if !file_path.starts_with(&dir) {
                tracing::warn!("Unauthorized attempt to read: {:?}", file_path);
                return Ok(Response::new(StatusCode::Forbidden));
            } else {
                return match Body::from_file(&file_path).await {
                    Ok(body) => Ok(Response::builder(StatusCode::Ok).body(body).build()),
                    Err(e) if e.kind() == io::ErrorKind::NotFound => {
                        tracing::warn!("File not found: {:?}", &file_path);
                        Ok(Response::new(StatusCode::NotFound))
                    }
                    Err(e) => Err(e.into()),
                };
            }
        }

        Ok(next.run(req).await)
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let args = match args::parse() {
        Ok(x) => x,
        Err(e) => {
            panic!("Args parsing error: {}", e);
        }
    };
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(args.database_conn)
        .connect_timeout(std::time::Duration::from_secs(3))
        .connect(args.database_url.as_str())
        .await
        .unwrap();
    if let Err(e) = pool.acquire().await {
        panic!(
            "Database connection failure {} url={}",
            e, args.database_url
        );
    };

    let state = State {
        db_pool: pool,
        static_dir: args.static_dir.clone(),
    };
    let mut app = tide::with_state(state);
    app.with(telemetry::TraceMiddleware::new());
    app.with(ServeMiddleware {});
    app.at("/").get(homepage::get);
    app.listen(args.addr.as_str()).await?;
    Ok(())
}
