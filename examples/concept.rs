extern crate wtdb;


use wtdb::connection::Conn;

fn main() {
    
    let mut db : Conn = Conn::new("127.0.0.1", "6942", "augustus", "augustus"); 

    let query_string : String = "SELECT * FROM testdb".into();
    
    let _ = db.execute(query_string.clone());
    let _ = db.execute(query_string.clone());
    let _ = db.execute(query_string);

    db.evt_log.log_print();
}
