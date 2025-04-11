#!/bin/bash
set -e

# 환경변수 파싱
for arg in "$@"; do
  case $arg in
    --EXECUTABLE=*)
      EXECUTABLE="${arg#*=}"
      shift
      ;;
    --LANGUAGE=*)
      LANGUAGE="${arg#*=}"
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

./"$EXECUTABLE" "$INPUT_FILE" "$OUTPUT_FILE" "$ANSWER_FILE"