# Client-Server RPC 구현

## 1. 개요

- 클라이언트와 서버 간의 원격 프로시저 호출(RPC) 구현
- Worker on Docker

## 2. 사용법
rustc 1.86.0에서 동작합니다.

- protopuf 설치
  - apt-get install protobuf-compiler
  - brew install protobuf
- docker-compose up
- cargo build
- cargo run --bin server
- cargo run --bin client

## 3. 구현

- 클라이언트
    - 서버로 파일명과 소스코드를 전송
- 서버
    - 파일을 생성
    - 워커에 소스코드 실행 요청
    - 실행 결과를 클라이언트에게 전송
- 워커
    - 소스코드 컴파일
    - 실행 결과를 서버에게 전송

## 4. 예시

```
💬 언어를 입력하세요 [c99, c++17, c++20, java8, python3, pypy]:
c++17
💬 코드를 입력하세요 (입력 완료 후 Enter):
#include <iostream>
using namespace std;
int main() {
        cout << "Hello World!" << endl;
        return 0;
}

실행 결과:
언어: c++17
Hello World!
```
```
💬 언어를 입력하세요 [c99, c++17, c++20, java8, python3, pypy]:
java8
💬 코드를 입력하세요 (입력 완료 후 Enter):
import java.util.*;
public class Main{
        public static void main(String args[]){
                System.out.println("Hello World!");
        }
}

실행 결과:
언어: java8
Hello World!
```
```
💬 언어를 입력하세요 [c99, c++17, c++20, java8, python3, pypy]:
python3
💬 코드를 입력하세요 (입력 완료 후 Enter):
print('Hello World!');

실행 결과:
언어: python3
Hello World!
```
