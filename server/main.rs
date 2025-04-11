use code_executor::executor_server::{Executor, ExecutorServer};
use code_executor::{CodeReply, CodeRequest};
use std::process::Command;
use std::{env, fs};
use tonic::{transport::Server, Request, Response, Status};

pub mod code_executor {
    tonic::include_proto!("code_executor");
}

#[derive(Default)]
pub struct MyExecutor {}

#[tonic::async_trait]
impl Executor for MyExecutor {
    async fn execute(&self, request: Request<CodeRequest>) -> Result<Response<CodeReply>, Status> {
        let req = request.into_inner();

        println!("📦 입력된 언어: {}", req.exec_lang);
        println!("📦 입력된 코드:\n{}", req.code);
        println!("📦 입력된 옵션: {}", req.option);

        let ext = match req.exec_lang.as_str() {
            "c99" => "c",
            "c++17" | "c++20" => "cc",
            "java8" => "java",
            "python3" | "pypy" => "py",
            _ => {
                return Err(Status::invalid_argument("지원하지 않는 언어입니다."));
            }
        };

        let cur_dir_path = env::current_dir()?
            .join("shared")
            .to_str()
            .unwrap()
            .to_string();

        // shared 디렉토리에 사용자의 소스코드를 Main.<ext>로 저장
        let cur_file_path = format!("{}/{}", cur_dir_path, format!("Main.{}", ext));
        fs::write(&cur_file_path, &req.code)
            .map_err(|e| Status::internal(format!("파일 저장 실패: {}", e)))?;

        let volume_arg = format!("{}:/app/shared", cur_dir_path);

        // [run, judge] 옵션에 따라 실행
        match req.option.as_str() {
            "run" => {
                println!("📦 실행 중...");
                let output = exec(volume_arg, req.exec_lang, None, None)
                    .map_err(|e| Status::internal(format!("실행 실패: {}", e)))?;

                println!("📦 실행 결과:\n{}", output);

                Ok(Response::new(CodeReply { result: output }))
            }
            "judge" => {
                println!("📦 채점 중...");
                Ok(Response::new(CodeReply {
                    result: String::from("채점 결과"),
                }))
            }
            _ => Err(Status::invalid_argument("지원하지 않는 옵션입니다.")),
        }
    }
}

fn exec(
    volume_arg: String,
    exec_lang: String,
    input_file: Option<String>,
    output_file: Option<String>,
) -> Result<String, Status> {
    let lang_arg = format!("--lang={}", exec_lang);
    let input_arg = input_file
        .map(|file| format!("--input={}", file))
        .unwrap_or_default();
    let output_arg = output_file
        .map(|file| format!("--output={}", file))
        .unwrap_or_default();

    let mut args = vec!["run", "--rm", "-v", &volume_arg, "worker", &lang_arg];
    if !input_arg.is_empty() {
        args.push(&input_arg);
    }
    if !output_arg.is_empty() {
        args.push(&output_arg);
    }

    println!("📦 Docker args: {:?}", args);
    let output = Command::new("docker")
        .args(args)
        .output()
        .map_err(|e| Status::internal(format!("Docker 실행 실패: {}", e)))?;

    if !output.status.success() {
        return Err(Status::internal(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;
    let executor = MyExecutor::default();

    println!("서버 실행 중 @ {}", addr);

    Server::builder()
        .add_service(ExecutorServer::new(executor))
        .serve(addr)
        .await?;

    Ok(())
}
