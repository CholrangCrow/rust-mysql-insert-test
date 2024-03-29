use mysql::prelude::Queryable; // exec_dorp()를 사용하기 위해 추가
use mysql::*;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Instant; // 환경 변수 사용을 위해 추가

fn main() {
    let start = Instant::now(); // 프로그램 시작 시간 측정

    // Open a file named 'log.txt' to write logs, create it if it does not exist
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    // 커맨드 라인 인자를 가져와서 URL로 사용
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the database URL as a command line argument.");
        eprintln!("'root:password@localhost:3306/database' is the format.");
        let duration = start.elapsed(); // 프로그램 실행 시간 측정
        let log_message = format!("Time elapsed for error end: {:?}", duration);
        println!("{}", log_message); // 표준 출력에 로그 출력
        writeln!(file, "{}", log_message).unwrap(); // 파일에 로그 작성;
        return;
    }
    let url = &args[1];
    println!("Database URL: {}", url);

    // url을 mysql://url 형식으로 변환
    let db_url = format!("{}{}", "mysql://", url);

    // OptsBuilder를 사용하여 Opts를 생성 (string으로 할 경우에는 안정성 보장 안됨.)
    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);

    // println!("Database builder: {:?}", builder);
    let pool = Pool::new(builder).unwrap();
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
