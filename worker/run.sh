#!/bin/bash
set -e

# 환경변수 파싱
for arg in "$@"; do
  case $arg in
    --LANGUAGE=*)
      LANGUAGE="${arg#*=}"
      shift
      ;;
    --EXECUTABLE=*)
      EXECUTABLE="${arg#*=}"
      shift
      ;;
    --INPUT_FILE=*)
      INPUT_FILE="${arg#*=}"
      shift
      ;;
    --OUTPUT_FILE=*)
      OUTPUT_FILE="${arg#*=}"
      shift
      ;;
    --ANSWER_FILE=*)
      ANSWER_FILE="${arg#*=}"
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
    RUN_CMD="./$EXECUTABLE"
    ;;
  "c++17")
    RUN_CMD="./$EXECUTABLE"
    ;;
  "c++20")
    RUN_CMD="./$EXECUTABLE"
    ;;
  "java8")
    RUN_CMD="java -Xms1024m -Xmx1920m -Xss512m -Dfile.encoding=UTF-8 ./$EXECUTABLE"
    ;;
  "python3")
    RUN_CMD="python3 -W ignore ./$EXECUTABLE.py"
    ;;
  "pypy")
    RUN_CMD="pypy3 -W ignore ./$EXECUTABLE.py"
    ;;
  *)
    echo "지원하지 않는 언어: $LANGUAGE"
    exit 1
    ;;
esac

if [[ -n "$INPUT_FILE" && -f "$INPUT_FILE" ]]; then
    RUN_CMD="$RUN_CMD < $INPUT_FILE"
fi

if [[ -n "$OUTPUT_FILE" ]]; then
    RUN_CMD="$RUN_CMD > $OUTPUT_FILE"
fi

eval "$RUN_CMD"
