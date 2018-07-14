use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::url_for;
use handlebars_iron::Template;
use mongo_db::Mongo;
use page_handler:: { check_authorized, authrized_handler, PageHandler };

pub struct Top {}

impl Top {
    pub fn new() -> Top {
        Top {}
    }
}

impl PageHandler for Top {
    fn is_get(&self) -> bool {
        true
    }

    fn is_post(&self) -> bool {
        false
    }

    fn path(&self) -> &str {
        ""
    }

    fn template(&self) -> &str {
        "index"
    }

    fn bind_url(&self) -> &str {
        "/"
    }

    fn handler(&self, req: &mut Request) -> IronResult<Response> {
        match check_authorized(req) {
            None => return authrized_handler(req),
            _ => {},
        }        

        let mut resp = Response::new();
        let mut data = HashMap::new();
        let mongo = Mongo::new();

        let url = format!("{}", url_for(req, "answer", HashMap::new()));
        let url_detail = format!("'{}'", format!("{}", url_for(req, "detail", HashMap::new())) + "?word=");
        let word_list = format!("[{}]",  mongo.get_translated_list());

        data.insert(String::from("translate_path"), url);
        data.insert(String::from("detail_path"), url_detail);
        data.insert(String::from("list"), word_list);
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

        Ok(resp)
    }
}