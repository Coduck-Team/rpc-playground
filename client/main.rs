use code_executor::executor_client::ExecutorClient;
use code_executor::CodeRequest;
use std::io::BufRead;

pub mod code_executor {
    tonic::include_proto!("code_executor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ExecutorClient::connect("http://[::1]:50051").await?;

    let stdin = std::io::stdin();

    println!("💬 언어를 입력하세요 [c99, c++17, c++20, java8, python3, pypy]:");
    let exec_lang: String = stdin.lock().lines().next().unwrap().unwrap();

    println!("💬 코드를 입력하세요 (입력 완료 후 Enter):");
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("입력 실패");

        if line.is_empty() {
            break;
        }

        lines.push(line);
    }
    let code = lines.join("\n");

    println!("⚙️ 실행 옵션을 입력하세요 [0: 입력 없이 실행, 1: 채점]");
    let option: u32 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    let request = tonic::Request::new(CodeRequest {
        exec_lang,
        code,
        option,
    });

    let response = client.execute(request).await?;

    println!("\n실행 결과:\n{}", response.into_inner().result);

    Ok(())
}
