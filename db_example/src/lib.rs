use std::ops::Not;
use postgres::{Client, Error, NoTls, Row};

pub fn helloworld() {
    let mut client = Client::connect("postgresql://nabbasi:Password1@localhost/memory_leak", NoTls)
        .expect("Unable to communicate with db_example");

    client.batch_execute("
        CREATE TABLE if not exists person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
        ").expect("Unable to create table");

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    ).unwrap_or(0);

    let rows = client.query("SELECT id, name, data FROM person where name = $1", &[&name]).unwrap_or(Vec::new());

    print_record(rows);

    client.batch_execute("drop table person").expect("Unable to drop table");
}

pub fn test() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://nabbasi:Password1@localhost/memory_leak", NoTls)?;

    client.batch_execute("
            CREATE TABLE IF NOT EXISTS author (
                id              SERIAL PRIMARY KEY,
                name            VARCHAR NOT NULL,
                country         VARCHAR NOT NULL
            )
        ")?;

    client.batch_execute("
            CREATE TABLE IF NOT EXISTS book  (
                id              SERIAL PRIMARY KEY,
                title           VARCHAR NOT NULL,
                author_id       INTEGER NOT NULL REFERENCES author
            )
        ")?;

    let result = client.query("SELECT id, name, country FROM author", &[]);

    print_record_using_result(result);

    client.batch_execute("drop table book").expect("Unable to drop book table");
    client.batch_execute("drop table author").expect("Unable to drop author table");

    return Ok(());
}

fn print_record_using_result(result: Result<Vec<Row>, Error>) {
    let rows = result.unwrap_or(vec![]);

    if rows.is_empty().not() {
        return;
    }

    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let country: &str = row.get(2);

        println!("found author: {} {} {:?}", id, name, country);
    }
}

fn print_record(rows: Vec<Row>) {
    for row in &rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
    
    println!("{}", check_vector(rows))
}

fn check_vector(vector: Vec<Row>) -> &'static str {
    // Match on the length of the vector using the `is_empty()` method
    match vector.is_empty() {
        true => "Vector is empty!",   // If vector is empty, return this message
        false => "Vector is not empty!" // If vector is not empty, return this message
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
