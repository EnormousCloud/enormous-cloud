pub mod proto;

use proto::{NetworkInfo, NetworkPropValue};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

// App and all its members should be Serializable by serde
#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    pub chains: Option<Vec<NetworkInfo>>,
}
impl App {
    pub fn new() -> Self {
        Self { chains: None }
    }
    pub fn from(chains: Vec<NetworkInfo>) -> Self {
        Self {
            chains: Some(chains.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <h1>"Enormous Cloud of Blockchains and Decentralized Finances"</h1>
                {self.render_chains()}
            </main>
        }
    }

    fn update(&mut self, _: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }
}

impl App {
    pub fn render_property(&self, kv: &NetworkPropValue) -> Node<Msg> {
        div(
            vec![class("network__property")],
            vec![
                div(vec![], vec![text(format!("{}:", &kv.label))]),
                div(vec![], vec![text(&kv.value)]),
            ],
        )
    }

    pub fn render_chain(&self, chain: &NetworkInfo) -> Node<Msg> {
        node! {
            <div class="network__item well">
                <h3>{text(&chain.name)} " Block Explorer"</h3>
                {div(vec![class("network__props")], {
                    chain.properties.iter()
                        .map(|p| { self.render_property(&p) })
                        .collect::<Vec<Node<Msg>>>()
                })}
                {if let Some(link) = &chain.link {
                    a(vec![href(link)], 
                        vec![text(format!("Explore {}", chain.name))])
                } else {
                    span(vec![], vec![])
                }}
            </div>
        }
    }

    pub fn render_chains(&self) -> Node<Msg> {
        let chains = match &self.chains {
            Some(x) => x,
            None => return node! { <div>"no chains"</div> },
        };
        if chains.len() == 0 {
            return div(vec![], vec![]);
        }
        div(vec![class("network__list")], {
            chains
                .iter()
                .map(|chain| self.render_chain(&chain))
                .collect::<Vec<Node<Msg>>>()
        })
    }
}

#[wasm_bindgen]
pub fn main(serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let mut app = App::new();
    if serialized_state.len() > 4 {
        match serde_json::from_str::<App>(&serialized_state) {
            Ok(state) => {
                app.chains = state.chains;
                // info!("parsing ok {:?}", app.chains);
            }
            Err(e) => {
                info!("parsing error {}", e);
            }
        };
    }
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
