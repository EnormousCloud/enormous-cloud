use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
}

// App and all its members should be Serializable by serde
#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    // blockchains: Vec<Blockchain>,
}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        info!("rendered App");
        trace!("trace App");
        node! {
            <main>
                <h1>"Enormous Cloud of Blockchains and Decentralized Finances"</h1>
            </main>
        }
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}

#[wasm_bindgen]
pub fn main(_serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let app = App{};
    // if let Ok(state) = serde_json::from_str::<App>(&serialized_state) {
    //     app.name = state.name;
    //     app.data = state.data;
    // };
    match web_sys::window() {
        Some(window) => {
            let document = window.document().expect("should have a document on window");
            Program::new_replace_mount(
                app,
                &document.query_selector_all("main").unwrap().get(0).unwrap(),
            );
        }
        None => {
            trace!("window not found");
            Program::mount_to_body(app);
        }
    }
}
