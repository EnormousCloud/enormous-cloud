use crate::State;
use client::App;
use sauron::prelude::*;
use serde_json;
use tide::{Request, Response, Result};

pub async fn get(req: Request<State>) -> Result {
    let mut res = Response::new(200);
    let file = format!("{}/index.html", req.state().static_dir);
    // let db_pool = req.state().db_pool.clone();
    // let mut conn = db_pool.acquire().await?;
    // TODO: fetch list of blockchain form database
    // let chains = db::get_chains(&mut conn).await;
   
    let content = std::fs::read_to_string(file.as_str())?;
    let app = App::new();
    let serialized_state = serde_json::to_string(&app).unwrap();

    let mut rendered = content.replace("main(``);", &format!("main(`{}`);", serialized_state));
    if let Some(start_tag) = rendered.find("<main>") {
        if let Some(end_tag) = rendered.find("</main>") {
            let before: String = rendered.chars().into_iter().take(start_tag).collect();
            let after: String = rendered.chars().into_iter().skip(end_tag + 7).collect();
            let mut buffer = String::new();
            app.view().render(&mut buffer).expect("must render");
            rendered = format!("{}{}{}", before, buffer, after);
        }
    }
    res.set_content_type("text/html");
    res.set_body(rendered.as_str());
    Ok(res)
}
