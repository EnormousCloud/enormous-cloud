use crate::{db, inject, State};
use client::App;
use sauron::prelude::*;
use serde_json;
use tide::{Request, Response, Result};

pub async fn get(req: Request<State>) -> Result {
    let mut res = Response::new(200);
    let file = format!("{}/index.html", req.state().static_dir);

    let db_pool = req.state().db_pool.clone();
    let mut conn = db_pool.acquire().await?;

    let links = db::get_links(&mut conn).await;
    tracing::info!("links {:?}", links);

    let props = crate::chainstate::remote_properties(links);
    let app = App::from(db::get_networks(&mut conn, Some(props)).await);
    tracing::info!("app {:?}", &app);
    let state_json = serde_json::to_string(&app).unwrap();

    let mut state_html = String::new();
    let content = std::fs::read_to_string(file.as_str())?;
    let rendered: String = match app.view().render(&mut state_html) {
        Ok(_) => {
            let c1 = inject::it(content.as_str(), "<main>", "</main>", &state_html);
            inject::replace(c1.as_str(), "main(`", "`)", "") // call to main function is removed
        }
        Err(_) => inject::it(content.as_str(), "main(`", "`)", &state_json),
    };

    res.set_content_type("text/html");
    res.set_body(rendered.as_str());
    Ok(res)
}
