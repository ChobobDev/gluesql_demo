use gluesql::prelude::*;

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "DROP TABLE IF EXISTS Glue;",
        "CREATE TABLE Glue (id INTEGER,langauge TEXT,taste TEXT);",
        "INSERT INTO Glue VALUES (1, 'Rust','Perfect'), (2, 'Java','Perfect'), (3, 'Go','Good'), (4, 'Javascript','edible');",
        "SELECT * FROM Glue WHERE taste='Perfect';",
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}

