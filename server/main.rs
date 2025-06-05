use code_executor::executor_server::{Executor, ExecutorServer};
use code_executor::{CodeReply, CodeRequest};
use home::home_dir;
use std::fs;
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

        println!("📦 입력된 언어: {}", req.language);
        println!("📦 입력된 코드:\n{}", req.source_code);
        println!("📦 입력된 옵션: {}", req.option);

        let ext = match req.language.as_str() {
            "c99" => "c",
            "c++17" | "c++20" => "cc",
            "python3" | "pypy3" => "py",
            _ => {
                return Err(Status::invalid_argument("지원하지 않는 언어입니다."));
            }
        };

        // home/shared 디렉토리에 사용자의 소스코드를 Main.<ext>로 저장
        let base_dir = home_dir().unwrap().join("coduck_data");

        let source_file = format!("Main.{}", ext);
        println!("📦 소스코드 파일 경로: {}", source_file);
        let source_file_path = base_dir
            .join(source_file.clone())
            .to_str()
            .unwrap()
            .to_string();

        println!("📦 소스코드 파일 저장 경로: {}", source_file_path);
        fs::write(&source_file_path, &req.source_code)
            .map_err(|e| Status::internal(format!("파일 저장 실패: {}", e)))?;

        let box_path = isolate_init_on_docker()
            .await
            .map_err(|e| Status::internal(format!("샌드박스 초기화 실패: {}", e)))?;
        println!("📦 샌드박스 경로: {}", box_path);

        // [run, judge] 옵션에 따라 실행
        match req.option.as_str() {
            "run" => {
                println!("📦 소스코드 파일 복사...");
                docker_cp(&source_file_path, &format!("{}{}", box_path, source_file))
                    .await
                    .map_err(|e| Status::internal(format!("파일 복사 실패: {}", e)))?;

                println!("📦 소스코드 컴파일...");
                let executable = compile_on_docker(req.language.clone(), &source_file)
                    .await
                    .map_err(|e| Status::internal(format!("소스코드 컴파일 실패: {}", e)))?;

                println!("📦 실행 파일: {}", executable);
                println!("📦 실행...");
                let output = execute_on_docker(req.language.clone(), &executable)
                    .await
                    .map_err(|e| Status::internal(format!("실행 실패: {}", e)))?;

                println!("📦 실행 결과:\n{}", output);
                Ok(Response::new(CodeReply { result: output }))
            }
            "judge" => {
                println!("📦 소스코드 컴파일...");
                compile_on_docker(req.language.clone(), &source_file)
                    .await
                    .map_err(|e| Status::internal(format!("소스코드 컴파일 실패: {}", e)))?;

                println!("📦 정해 컴파일...");
                compile_on_docker("c++17".to_string(), "solution.cpp")
                    .await
                    .map_err(|e| Status::internal(format!("정해 컴파일 실패: {}", e)))?;

                println!("📦 제너레이터 컴파일...");
                compile_on_docker("c++17".to_string(), "testlib/generator.cpp")
                    .await
                    .map_err(|e| Status::internal(format!("제너레이터 컴파일 실패: {}", e)))?;

                let num_test_cases = 3;
                println!("📦 데이터 생성...");
                for i in 0..num_test_cases {
                    let input_file = format!("input/{}.in", i);

                    generate_data_on_docker(
                        "testlib/generator.cpp".to_string(),
                        42 + i as u32, // 랜덤 시드
                        input_file,
                    )
                    .await
                    .map_err(|e| Status::internal(format!("데이터 생성 실패: {}", e)))?;
                }

                println!("📦 체커 컴파일...");
                compile_on_docker("c++17".to_string(), "testlib/checker.cpp")
                    .await
                    .map_err(|e| Status::internal(format!("체커 컴파일 실패: {}", e)))?;

                println!("📦 채점...");
                let mut result = String::new();
                for i in 0..num_test_cases {
                    let input_file = format!("input/{}.in", i);
                    let output_file = format!("output/{}.out", i);
                    let answer_file = format!("answer/{}.out", i);

                    // execute_on_docker(
                    //     req.language.clone(),
                    //     "Main".to_string(),
                    //     Some(input_file.clone()),
                    //     Some(output_file.clone()),
                    // )
                    // .await
                    // .map_err(|e| Status::internal(format!("소스코드 실행 실패: {}", e)))?;
                    //
                    // execute_on_docker(
                    //     "c++17".to_string(),
                    //     "solution".to_string(),
                    //     Some(input_file.clone()),
                    //     Some(answer_file.clone()),
                    // )
                    // .await
                    // .map_err(|e| Status::internal(format!("정해 실행 실패: {}", e)))?;

                    let verdict = judge_on_docker(
                        "c++17".to_string(),
                        "testlib/checker".to_string(),
                        input_file,
                        output_file,
                        answer_file,
                    )
                    .await
                    .map_err(|e| Status::internal(format!("체커 실행 실패: {}", e)))?;

                    result.push_str(&format!("테스트 케이스 {}: {}\n", i, verdict));
                }

                Ok(Response::new(CodeReply {
                    result: format!("채점 완료! 결과:\n{}", result),
                }))
            }
            _ => Err(Status::invalid_argument("지원하지 않는 옵션입니다.")),
        }
    }
}

async fn docker_cp(src_path: &str, dest_path: &str) -> Result<(), Status> {
    Command::new("docker")
        .args([
            "cp",
            src_path,
            format!("coduck-grader:{}", dest_path).as_str(),
        ])
        .output()
        .map_err(|e| Status::internal(format!("Docker cp 실행 실패: {}", e)))?;

    Ok(())
}

async fn isolate_init_on_docker() -> Result<String, Status> {
    let args = vec![
        "exec",
        "-u",
        "judge",
        "coduck-grader",
        "isolate",
        "--tty-hack",
        "--init",
    ];

    let output = Command::new("docker")
        .args(&args)
        .output()
        .map_err(|e| Status::internal(format!("Docker 실행 실패: {}", e)))?;

    let box_path = output
        .stdout
        .split(|&b| b == b'\n')
        .next()
        .ok_or_else(|| Status::internal("샌드박스 초기화 실패: 경로를 찾을 수 없습니다."))?;

    Ok(format!("{}/box/", String::from_utf8_lossy(box_path)))
}

async fn compile_on_docker(language: String, source_file: &str) -> Result<String, Status> {
    let args = vec![
        "exec",
        "-u",
        "judge",
        "coduck-grader",
        "isolate",
        "--run",
        "--processes=4",
        "--full-env",
        "--",
    ];

    let executable = source_file
        .split('.')
        .next()
        .ok_or_else(|| Status::invalid_argument("소스코드 파일 이름이 잘못되었습니다."))?;

    let python_temp = format!(
        "\"import py_compile; py_compile.compile(r'{}')\"",
        source_file
    );
    let command = match language.as_str() {
        "c99" => vec![
            "/usr/bin/gcc",
            source_file,
            "-o",
            executable,
            "-O2",
            "-Wall",
            "-lm",
            "-static",
            "-std=gnu99",
        ],
        "c++17" => vec![
            "/usr/bin/g++",
            source_file,
            "-o",
            executable,
            "-O2",
            "-Wall",
            "-lm",
            "-static",
            "-std=gnu++17",
        ],
        "c++20" => vec![
            "/usr/bin/g++",
            source_file,
            "-o",
            executable,
            "-O2",
            "-Wall",
            "-lm",
            "-static",
            "-std=gnu++20",
        ],
        "python3" => vec!["/usr/bin/python3", "-W", "ignore", "-c", &python_temp],
        "pypy3" => vec!["/usr/bin/pypy3", "-W", "ignore", "-c", &python_temp],
        _ => {
            let result = Err(Status::invalid_argument("지원하지 않는 언어입니다."));
            return result;
        }
    };

    Command::new("docker")
        .args(args)
        .args(command)
        .output()
        .map_err(|e| Status::internal(format!("Docker 실행 실패: {}", e)))?;

    if language == "python3" || language == "pypy3" {
        return Ok(String::from(source_file));
    }

    Ok(executable.to_string())
}

async fn execute_on_docker(language: String, executable: &str) -> Result<String, Status> {
    let args = vec![
        "exec",
        "-u",
        "judge",
        "coduck-grader",
        "isolate",
        "--run",
        "--",
    ];

    let command = match language.as_str() {
        "c99" | "c++17" | "c++20" => vec![executable],
        "python3" => vec!["/usr/bin/python3", executable],
        "pypy3" => vec!["/usr/bin/pypy3", executable],
        _ => {
            return Err(Status::invalid_argument("지원하지 않는 언어입니다."));
        }
    };

    let output = Command::new("docker")
        .args(args)
        .args(command)
        .output()
        .map_err(|e| Status::internal(format!("Docker 실행 실패: {}", e)))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

async fn generate_data_on_docker(
    generator_file: String,
    random_seed: u32,
    input_file: String,
) -> Result<String, Status> {
    todo!()
}
async fn judge_on_docker(
    language: String,
    checker_file: String,
    input_file: String,
    output_file: String,
    answer_file: String,
) -> Result<String, Status> {
    todo!()
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
