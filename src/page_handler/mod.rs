use reqwest;
use serde_json;
use serde_urlencoded;
use iron::prelude::*;
use iron::headers::Cookie;
use session::Session;
use mongo_db::{ Mongo, convert_jsonlist_to_string };
use translate_result::TranslateResult;

pub mod top;
pub mod translate;
pub mod detail;
pub mod auth;

pub trait PageHandler{
    fn is_get(&self) -> bool;
    fn is_post(&self) -> bool;
    fn path(&self) -> &str;
    fn template(&self) -> &str;
    fn bind_url(&self) -> &str;
    fn handler(&self, req: &mut Request) -> IronResult<Response>;
}

pub fn get_session() -> Session {
    //セッション管理オブジェクト
    let sess = match Session::new() {
        Some(sess) => sess,
        None => {
            panic!("fatal session cannot create.");
        },
    };

    sess
}

pub fn check_authorized(req: &mut Request) -> Option<()> {
    match req.headers.get() {
        Some(&Cookie(ref cookie)) => {
            let mut c_cokkie = cookie.clone();
            match c_cokkie.pop() {
                Some(str_cookie) => {
                    //cookie:"x-auth=hoge"
                    //cookieの区切りは[;] or [,]
                    let sess = get_session();
                    let cookies: Vec<&str> = str_cookie.split(|c: char| c == ';' || c == ',').collect();
                    for val in cookies.into_iter() {
                        if sess.exists_session(val.to_string().replace("xauth=", "")) {
                            return Some(())
                        }
                    }
                    return None
                },
                _ => return None
            };
        },
        _ => return None
    }
}

pub fn authrized_handler(req: &mut Request) -> IronResult<Response> {
    let auth = auth::Auth::new();
    auth.handler(req)
}

pub fn get_translate_result(word: String) -> String {
    let mongo = Mongo::new();
    //既に登録してる場合
    if let Ok(count) = mongo.check_exists(word.clone()) {
        if count > 0 {
            if let Ok(data) = mongo.get_json(word.clone()){
                println!("read from mongodb phrase:{}", word.clone());
                return convert_jsonlist_to_string(&data, 999);
            }
        }
    }

    //登録が無いため WEB APIで取得する。
    let enc_word = match serde_urlencoded::to_string(word.clone()) {
        Ok(word) => word.trim().to_string(),
        Err(_e) => word.trim().to_string()
    };

    let url = format!("https://glosbe.com/gapi/translate?from=en&dest=ja&format=json&phrase={}&pretty=false", enc_word);
    let json_str = match reqwest::get(url.as_str()) {
        Ok(mut req) => {
            match req.text() {
                Ok(t) => t,
                Err(e) => return format!("'get error(to text) {}'", e.to_string())
            }
        },
        Err(e) => return format!("'get error {}'", e.to_string())
    };


    let data: TranslateResult = match serde_json::from_str(json_str.as_str()) {
        Ok(j) => j,
        Err(e) => return format!("'serialize error {}'", e.to_string())
    };

    //結果の整形        
    if let Some(phrase) = data.phrase.clone() {
        if data.tuc.len() > 0  {
            match mongo.check_exists(phrase) {
                Ok(count) => {
                    println!("record count:{}", count);
                    if count == 0 {
                        //save to mongodb
                        match mongo.save_json(&data) {
                            Ok(()) => {},
                            Err(e) => return e.to_string()
                        }
                    }
                },
                Err(e) => return e
            }
        }
    }

    convert_jsonlist_to_string(&data, 999)
}