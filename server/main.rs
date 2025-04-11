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

        let ext = match req.exec_lang.as_str() {
            "c99" => "c",
            "c++17" | "c++20" => "cc",
            "java8" => "java",
            "python3" | "pypy" => "py",
            _ => {
                return Err(Status::invalid_argument("지원하지 않는 언어입니다."));
            }
        };

        let current_dir = env::current_dir()?;
        let abs_path = current_dir.join("shared").to_str().unwrap().to_string();
        let volume_arg = format!("{}:/app/shared", abs_path);

        let filename = format!("Main.{}", ext);
        let path = format!("{}/{}", abs_path, filename);
        fs::write(&path, &req.code)
            .map_err(|e| Status::internal(format!("파일 저장 실패: {}", e)))?;

        let output = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-v",
                &volume_arg,
                "cpp-python-worker",
                &req.exec_lang,
            ])
            .output()
            .map_err(|e| Status::internal(format!("Docker 실행 실패: {}", e)))?;

        if !output.status.success() {
            return Err(Status::internal(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(Response::new(CodeReply {
            result: String::from_utf8_lossy(&output.stdout).to_string(),
        }))
    }
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
