extern crate wtdb;


use wtdb::connection::Conn;

fn main() {
    
    let mut db : Conn = Conn::new("127.0.0.1", "6942", "jack", "jack"); 

    let query_string : String = "SELECT * FROM Customers;".into();
    let _ = db.execute(query_string);

    let query_string : String = "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country)VALUES ('Cardinal','Tom B. Erichsen','Skagen 21','Stavanger','4006','Norway');".into();
    let _ = db.execute(query_string);
    

    let query_string : String = "UPDATE Customers SET ContactName='Alfred Schmidt', City='Frankfurt' WHERE CustomerID=1;".into();
    let _ = db.execute(query_string);

    db.evt_log.log_print();
}
