use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::url_for;
use handlebars_iron::Template;
use mongo_db::Mongo;
use page_handler:: { check_authorized, authrized_handler };
use serde_json;
use page_handler::PageHandler;

pub struct Detail{}

impl Detail {
    pub fn new() -> Detail {
        Detail {}
    }

    fn get_translate_all(&self, word: String) -> Result<String, String> {
        let mongo = Mongo::new();

        match mongo.get_raw_json(word) {
            Ok(data) => {
                Ok(serde_json::to_string(&data).unwrap())
            },
            Err(e) => { 
                println!("parse error.{}", e);
                Err(e)
            },
        }
    }
}

impl PageHandler for Detail {
    fn is_get(&self) -> bool {
        true
    }

    fn is_post(&self) -> bool {
        false
    }

    fn template(&self) -> &str {
        "detail"
    }

    fn bind_url(&self) -> &str {
        "/detail"
    }

    fn path(&self) -> &str {
        "detail"
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

        let word = match map.find(&["word"]) {
            Some(&Value::String(ref word)) => { 
                word.to_string()
            },
            _ => "".to_string()
        };

        let json_obj = match self.get_translate_all(word){
            Ok(data) => data,
            Err(e) => 
            {
                println!("error at get_translate_all. {}", e);
                return Ok(resp);
            }
        };
        let json_str = json_obj.to_string().replace("\"", "'");

        data.insert(String::from("translate_path"), url_ans);
        data.insert(String::from("list_path"), url_index);
        data.insert(String::from("message"), json_str);
        resp.set_mut(Template::new("detail", data)).set_mut(status::Ok);

        Ok(resp)
    }

}
    
