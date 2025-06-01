# Client-Server RPC 구현

## 1. 개요

- 클라이언트와 서버 간의 원격 프로시저 호출(RPC) 구현
- [Grader on Docker](https://github.com/Coduck-Team/grader-docker)
    - [ioi/isolate](https://github.com/ioi/isolate)를 사용한 채점 시스템

## 2. 사용법

rustc 1.86.0에서 동작합니다.

- protobuf 설치
    - apt-get install protobuf-compiler
    - brew install protobuf
- docker compose -f grader-docker/docker-compose.yml up -d
- cargo build
- cargo run --bin server
- cargo run --bin client

## 3. 채점 구현

- 클라이언트
    - 서버로 파일명과 소스코드를 전송
- 서버
    - 파일을 생성
    - 워커 작업 요청
        - 소스코드 컴파일
        - 소스코드 실행
        - 제너레이터 실행
        - 체커 실행
    - 실행 결과를 클라이언트에게 전송
- 워커
    - 요청 받은 작업 실행
    - 실행 결과를 서버에게 전송

## 4. 채점 예시

```
💬 언어를 입력하세요 [c99, c++17, c++20, java8, python3, pypy]:
c++17
💬 코드를 입력하세요 (입력 완료 후 Enter):
#include <iostream>
using namespace std;
int main() {
        int a, b;
        cin >> a >> b;
        cout << a + b << endl;
        return 0;
}

⚙️ 실행 옵션을 입력하세요 [run, judge]:
judge

실행 결과:
채점 완료! 결과:
테스트 케이스 0: ok "907"

테스트 케이스 1: ok "391"

테스트 케이스 2: ok "955"

```

```
💬 언어를 입력하세요 [c99, c++17, c++20, java8, python3, pypy]:
java8
💬 코드를 입력하세요 (입력 완료 후 Enter):
import java.util.*;
public class Main{
        public static void main(String args[]){
                Scanner sc = new Scanner(System.in);
                int a, b;
                a = sc.nextInt();
                b = sc.nextInt();
                System.out.println(a + b);
        }
}

⚙️ 실행 옵션을 입력하세요 [run, judge]:
judge

실행 결과:
채점 완료! 결과:
테스트 케이스 0: ok "907"

테스트 케이스 1: ok "391"

테스트 케이스 2: ok "955"
```

```
💬 언어를 입력하세요 [c99, c++17, c++20, java8, python3, pypy]:
python3
💬 코드를 입력하세요 (입력 완료 후 Enter):
print(sum(map(int, input().split())))

⚙️ 실행 옵션을 입력하세요 [run, judge]:
judge

실행 결과:
채점 완료! 결과:
테스트 케이스 0: ok "907"

테스트 케이스 1: ok "391"

테스트 케이스 2: ok "955"

```
