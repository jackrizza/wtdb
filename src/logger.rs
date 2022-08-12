
#[derive(Debug, Clone)]
pub enum EvtType {
    Succsess(String),
    Info(String),
    Warning(String),
    Error(String),
    None

}

pub struct Evt {
    pub id : i32,
    pub evt_type : EvtType,
    pub viewed : bool,
}

impl Evt {
    pub fn new(id : i32, evt_type : EvtType) -> Self {
        Evt {
            id : id,
            evt_type : evt_type,
            viewed : false,
        }
    }
    pub fn viewed(&mut self) {
        self.viewed = true;
    }
}

pub struct EvtLog {
    current_id : i32,
    pub log : Vec<Evt>,
}

impl EvtLog {
    pub fn new() -> Self {
        EvtLog {
            current_id : 0,
            log : Vec::new(),
        }
    }
    pub fn add(&mut self, evt : EvtType) {
        self.current_id += 1;
        self.log.push(Evt::new(self.current_id, evt));
    }
    
    pub fn lookup(&mut self, id : i32) -> usize {
        let lookup = |x : &Vec<Evt>| -> usize {
            x
                .into_iter()
                .position(|x| x.id == id)
                .unwrap()
                .clone()

        };

        lookup(&self.log)

    }
    
    pub fn viewed(&mut self, id : i32) {
        let lookup = &self.lookup(id);
        self.log[lookup.clone()].viewed();
    }

    pub fn log_print(self) {
        for evt in self.log {
            match evt.evt_type {
                EvtType::Succsess(s) => println!("Succsess : {}", s),
                EvtType::Info(i) => println!("Info : {}", i),
                EvtType::Warning(w) => println!("Warning : {}", w),
                EvtType::Error(e) => println!("Error : {}", e),
                _ => println!("Type not matched"),
            }
        }
    }
}
