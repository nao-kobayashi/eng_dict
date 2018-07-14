use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::url_for;
use handlebars_iron::Template;
use page_handler:: { check_authorized, authrized_handler, PageHandler, get_translate_result };


pub struct Translate {}

impl Translate {
    pub fn new() -> Translate {
        Translate {}
    }
}

impl PageHandler for Translate {
    fn is_get(&self) -> bool {
        false
    }

    fn is_post(&self) -> bool {
        true
    }

    fn path(&self) -> &str {
        "answer"
    }

    fn template(&self) -> &str {
        "answer"
    }

    fn bind_url(&self) -> &str {
        "/answer"
    }

    fn handler(&self, req: &mut Request) -> IronResult<Response> {
        match check_authorized(req) {
            None => return authrized_handler(req),
            _ => {},
        }

        use params::{Params, Value};
        let mut resp = Response::new();
        let mut data = HashMap::new();

        let url_ans = format!("{}", url_for(req, "answer", HashMap::new()));
        let url_index = format!("{}", url_for(req, "index", HashMap::new()));
        let map = &req.get_ref::<Params>().unwrap();

        let message = match map.find(&["word"]) {
            Some(&Value::String(ref word)) => { 
                get_translate_result(word.to_string())
            },
            _ => "".to_string()
        };

        data.insert(String::from("translate_path"), url_ans);
        data.insert(String::from("list_path"), url_index);
        data.insert(String::from("message"), format!("[{}]", message));
        resp.set_mut(Template::new("answer", data)).set_mut(status::Ok);

        Ok(resp)
    }
}