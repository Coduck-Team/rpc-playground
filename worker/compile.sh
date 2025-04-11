#!/bin/bash
set -e

# 환경변수 파싱
for arg in "$@"; do
  case $arg in
    --LANGUAGE=*)
      LANGUAGE="${arg#*=}"
      shift
      ;;
    --SOURCE_FILE=*)
      SOURCE_FILE="${arg#*=}"
      shift
      ;;
    --EXECUTABLE=*)
      EXECUTABLE="${arg#*=}"
      shift
      ;;
    *)
      echo "Unknown option: $arg"
      ;;
  esac
done

cd /app/shared

case "$LANGUAGE" in
  "c99")
    gcc "$SOURCE_FILE" -o "$EXECUTABLE" -O2 -Wall -lm -static -std=gnu99
    ;;
  "c++17")
    g++ "$SOURCE_FILE" -o "$EXECUTABLE" -O2 -Wall -lm -static -std=gnu++17 -DONLINE_JUDGE -DBOJ
    ;;
  "c++20")
    g++ "$SOURCE_FILE" -o "$EXECUTABLE" -O2 -Wall -lm -static -std=gnu++20 -DONLINE_JUDGE -DBOJ
    ;;
  "java8")
    javac -J-Xms1024m -J-Xmx1920m -J-Xss512m -encoding UTF-8 "$SOURCE_FILE"
    ;;
  "python3")
    python3 -W ignore -c "import py_compile; py_compile.compile(r'$SOURCE_FILE')"
    ;;
  "pypy")
    pypy3 -W ignore -c "import py_compile; py_compile.compile(r'$SOURCE_FILE')"
    ;;
  *)
    echo "지원하지 않는 언어: $LANGUAGE"
    exit 1
    ;;
esac
