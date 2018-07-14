use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use redis;
use redis::Commands;
use redis::{ ToRedisArgs, FromRedisValue};

pub struct Session {
    #[allow(dead_code)] 
    client: redis::Client,
    conn: redis::Connection,
}

impl Session {
    pub fn new() -> Option<Session> {
        let cli = match redis::Client::open("redis://192.168.56.2/") {
            Ok(cl) => cl,
            Err(e) => { 
                println!("redis open error {:?}", e);
                return None;
            },
        };

        let con = match cli.get_connection() {
            Ok(con) => con,
            Err(e) => {
                println!("redis get connection error {:?}", e);
                return None
            },
        };

        Some(Session {
            client: cli,
            conn: con,
        })
    }

    //有効なセッションかどうか
    pub fn exists_session(&self, session_id: String) -> bool {
        match self.get_redis_value::<String, String>(session_id.clone()) {
            Ok(_) => {
                //println!("session id existed {:?}", session_id.clone());
                return true
            },
            Err(e) => {
                println!("exists session error {}", e);
                return false
            },
        }
    }

    pub fn create_session(&self) -> Option<String> {
        //ランダムな16桁の文字列を生成
        let session_id = thread_rng().sample_iter(&Alphanumeric).take(16).collect::<String>();

        //セッションIDとしてredisにセット 30分でexpireする。
        match self.set_redis_session(session_id.clone()) {
            Ok(_) => {
                match self.set_redis_expire(session_id.clone()) {
                    Ok(_) => {
                        //println!("session id created {:?}", session_id.clone());
                    },
                    Err(e) => {
                        println!("redis expire error {:?}", e);
                        return None;
                    },
                }
            },
            Err(e) => {
                println!("redis set error {:?}", e);
                return None;
            },
        }

        Some(session_id)
    }

    fn get_redis_value<K, V>(&self, key: K) -> Result<V, redis::RedisError> where K: ToRedisArgs, V: FromRedisValue {
        let res: V = try!(self.conn.get(key));
        Ok(res)
    }

    fn set_redis_expire(&self, session_id: String) -> redis::RedisResult<()> {
        let _: () = try!(self.conn.expire(session_id, 1800));
        Ok(())
    }

    fn set_redis_session(&self, session_id: String) -> redis::RedisResult<()> {
        let _: () = try!(self.conn.set(session_id, 1));
        Ok(())
    }

}

#[test]
fn create_session() {
    let sess = Session::new().expect("cannot create session");
    
    if let Some(x) = sess.create_session() {
        if !sess.exists_session(x) {
            panic!("not equal session id.");
        }
    } else {
        panic!("cannot generate session_id.");
    }
}