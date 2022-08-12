use crate::logger;
use ureq::{Agent, AgentBuilder, Error};
use std::time::Duration;


pub struct Conn {
    url : String,
    port : String,
    username : String,
    password : String, // TODO : I don't like passwords
    pub evt_log : logger::EvtLog,
}



impl Conn {
    pub fn new(url : &str, port : &str, username : &str, password : &str) -> Self {
        Conn {
            url : url.into(),
            port : port.into(),
            username : username.into(),
            password : password.into(),
            evt_log : logger::EvtLog::new(),
        }

    }

    pub fn execute(&mut self, query : String) -> logger::EvtType  {

        let uri = format!("http://{}:{}/execute", self.url, self.port);
        self.evt_log.add(match ureq::post(&uri)
            .send_json(ureq::json!({ "query" : &query })) {
            Ok(response) => logger::EvtType::Succsess(format!("{:?}", response)),
            Err(Error::Status(code, response)) => logger::EvtType::Error(format!("{:?} - {:?}", code , response)),
            Err(_) => logger::EvtType::Error(format!("Unkown Error")),
        });
        
        self.evt_log.log[self.evt_log.log.len() -1].evt_type.clone()

        }


}
