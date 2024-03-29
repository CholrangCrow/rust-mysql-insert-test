use mysql::prelude::Queryable; // exec_dorp()를 사용하기 위해 추가
use mysql::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let start = Instant::now(); // 프로그램 시작 시간 측정

    let url = "mysql://root:password@localhost:3306/database";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Create a table named 'test' if it does not exist
    conn.exec_drop(
        r"CREATE TABLE IF NOT EXISTS test (
            id INT AUTO_INCREMENT PRIMARY KEY,
            value INT
        )",
        (),
    )
    .unwrap();

    // Open a file named 'log.txt' to write logs, create it if it does not exist
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    for _ in 0..10000 {
        let value: i32 = rand::random();
        let query_start = Instant::now(); // 쿼리 시작 시간 측정
        conn.exec_drop(
            r"INSERT INTO test (value) VALUES (:value)",
            params! {
                "value" => value
            },
        )
        .unwrap();
        let duration = query_start.elapsed(); // 쿼리 실행 시간 측정
        let log_message = format!("Time elapsed for query is: {:?}", duration);
        println!("{}", log_message); // 표준 출력에 로그 출력
        writeln!(file, "{}", log_message).unwrap(); // 파일에 로그 작성
    }
    let duration = start.elapsed(); // 프로그램 실행 시간 측정
    let log_message = format!("Time elapsed for the entire program is: {:?}", duration);
    println!("{}", log_message); // 표준 출력에 로그 출력
    writeln!(file, "{}", log_message).unwrap(); // 파일에 로그 작성
}
