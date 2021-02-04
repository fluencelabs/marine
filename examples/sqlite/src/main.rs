/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use fluence::fce;
use fce_sqlite_connector;
use fce_sqlite_connector::State;

pub fn main() {}

#[fce]
pub fn test1(age: i64) {
    let connection = fce_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('Bob', 69);
        ",
        )
        .unwrap();

    let mut statement = connection
        .prepare("SELECT * FROM users WHERE age > ?")
        .unwrap();

    statement.bind(1, age).unwrap();

    while let State::Row = statement.next().unwrap() {
        println!("name = {}", statement.read::<String>(0).unwrap());
        println!("age = {}", statement.read::<i64>(1).unwrap());
    }
}

#[fce]
pub fn test2(age: i64) {
    use fce_sqlite_connector::Value;

    let connection = fce_sqlite_connector::open(":memory:").unwrap();

    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('Bob', 69);
        ",
        )
        .unwrap();

    let mut cursor = connection
        .prepare("SELECT * FROM users WHERE age > ?")
        .unwrap()
        .cursor();

    cursor.bind(&[Value::Integer(age)]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        println!(
            "name = {}",
            row[0].as_string().expect("error on row[0] parsing")
        );
        println!(
            "age = {}",
            row[1].as_integer().expect("error on row[1] parsing")
        );
    }
}

#[fce]
pub fn test3() {
    let db_path = "/var/users.sqlite";
    let connection = fce_sqlite_connector::open(db_path).expect("error on connection establishing");

    let execute_result = connection.execute(
        "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('Bob', 69);
        ",
    );

    println!("execute result: {:?}", execute_result);

    //TODO fix it
    // let file_size = std::fs::metadata(db_path).expect("error on file_size check").len();
    // println!("{} file size is {}", db_path, file_size);
}
