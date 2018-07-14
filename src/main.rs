extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate params;
extern crate serde_urlencoded;
extern crate reqwest;
extern crate eng_dict;
extern crate serde_json;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use iron::Error;
use router::Router;
use handlebars_iron::{HandlebarsEngine, DirectorySource};
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use eng_dict::page_handler::PageHandler;
use eng_dict::page_handler::auth::Auth;
use eng_dict::page_handler::detail::Detail;
use eng_dict::page_handler::translate::Translate;
use eng_dict::page_handler::top::Top;

fn create_handlers() -> Vec<Box<PageHandler>> {
    let handlers: Vec<Box<PageHandler>> = vec![
        Box::new(Top::new()),
        Box::new(Auth::new()),
        Box::new(Detail::new()),
        Box::new(Translate::new())
    ];

    handlers
}

fn main() {
    //request controller
    fn controller(req: &mut Request) -> IronResult<Response> {
        let handlers = create_handlers();
        //if url not registered. show auth page 
        let auth: Box<PageHandler> = Box::new(Auth::new());
        let target: _ = {
            let path_str = req.url.path()[0];
            let handler = if let Some(handler) = handlers.as_slice().iter().filter(|h| h.path() == path_str).next() {
                handler
            } else {
                &auth
            };
            handler
        };
        target.handler(req)
    }

    //Create Router
    let mut router = Router::new();    
    for handler in create_handlers() {
        if handler.is_get() {
            router.get(handler.bind_url(), controller, handler.template());
        }

        if handler.is_post() {
            router.post(handler.bind_url(), controller, handler.template());
        }
    }
    
    //crate mount
    let mut mount = Mount::new();
    mount.mount("/", router)
        .mount("/css/", Static::new(Path::new("./src/css")));

    //Create Chain
    let mut chain = Chain::new(mount);

    // Add HandlerbarsEngine to middleware Chain
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./src/templates/", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }
    chain.link_after(hbse);

    println!("Listen on localhost:3000");
    let iron_instance = Iron::new(chain);
    println!("worker threads:{}", iron_instance.threads);
    iron_instance.http("localhost:3000").unwrap();
}