use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::url_for;
use handlebars_iron::Template;
use mongo_db::Mongo;
use page_handler:: { PageHandler, get_session };
use iron::headers::SetCookie;

pub struct Auth {}

impl Auth {
    pub fn new() -> Auth {
        Auth {}
    }
}

impl PageHandler for Auth {
    fn is_get(&self) -> bool {
        true
    }

    fn is_post(&self) -> bool {
        true
    }

    fn template(&self) -> &str {
        "auth"
    }

    fn bind_url(&self) -> &str {
        "/auth"
    }

    fn path(&self) -> &str {
        "auth"
    }

    fn handler(&self, req: &mut Request) -> IronResult<Response> {
        use params::{Params, Value};
        let mut resp = Response::new();
        let mut data = HashMap::new();

        let url = format!("{}", url_for(req, "auth", HashMap::new()));
        let url_answer = format!("{}", url_for(req, "answer", HashMap::new()));
        let url_detail = format!("'{}'", format!("{}", url_for(req, "detail", HashMap::new())) + "?word=");
        let map = &req.get_ref::<Params>().unwrap();

        let user = match map.find(&["user"]) {
            Some(&Value::String(ref user)) => { 
                user.to_string()
            },
            _ => "".to_string()
        };

        let pass = match map.find(&["pass"]) {
            Some(&Value::String(ref pass)) => { 
                pass.to_string()
            },
            _ => "".to_string()
        };

        let mongo = Mongo::new();
        let count = match mongo.check_account(user.to_lowercase(), pass.to_lowercase()) {
            Some(count) => count,
            None => 0
        };

        println!("{} {} {}", user, pass, count);

        if count > 0 {
            let sess = get_session();

            if let Some(id) = sess.create_session() {
                let cookie = SetCookie(vec![String::from(format!("xauth={}", id))]);
                resp.headers.set(cookie);

                let word_list = format!("[{}]",  mongo.get_translated_list());
                data.insert(String::from("translate_path"), url_answer);
                data.insert(String::from("detail_path"), url_detail);
                data.insert(String::from("list"), word_list);
                resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

                Ok(resp)
            } else {
                panic!("fatal session id cannot generate.");
            }

        } else {

            data.insert(String::from("authrized_path"), url);
            resp.set_mut(Template::new("auth", data)).set_mut(status::Ok);
            Ok(resp)

        }
    }

}