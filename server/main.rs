use code_executor::executor_server::{Executor, ExecutorServer};
use code_executor::{CodeReply, CodeRequest};
use std::fs;
use std::path::Path;
use std::process::Command;
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

        println!("📦 입력된 언어: {}", req.filename);
        println!("📦 입력된 코드:\n{}", req.code);

        // 파일 저장
        let path = format!("./shared/{}", req.filename);
        println!("path: {}", path);

        let ext = Path::new(&path)
            .extension()
            .ok_or_else(|| Status::invalid_argument("파일 확장자가 없습니다."))?
            .to_str()
            .ok_or_else(|| Status::invalid_argument("파일 확장자 변환 실패"))?;

        fs::write(&path, &req.code)
            .map_err(|e| Status::internal(format!("파일 저장 실패: {}", e)))?;

        // 컴파일
        match ext {
            "cpp" => {
                Command::new("g++")
                    .arg(&path)
                    .arg("-o")
                    .arg(format!("./shared/{}.out", req.filename).as_str())
                    .output()
                    .map_err(|e| Status::internal(format!("컴파일 실패: {}", e)))?;
            }
            "py" => {}
            _ => {
                return Err(Status::invalid_argument("지원하지 않는 파일 형식입니다."));
            }
        }

        // 실행
        let output = match ext {
            "cpp" => Command::new(format!("./shared/{}.out", req.filename).as_str()).output(),
            "py" => Command::new("python").arg(&path).output(),
            _ => unreachable!(),
        }
        .map_err(|e| Status::internal(format!("실행 실패: {}", e)))?;

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
