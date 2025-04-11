#!/bin/bash
set -e


for arg in "$@"; do
  case $arg in
    --EXECUTABLE=*)
      EXECUTABLE="${arg#*=}"
      shift
      ;;
    --RANDOM_SEED=*)
      RANDOM_SEED="${arg#*=}"
      shift
      ;;
    --INPUT_FILE=*)
      INPUT_FILE="${arg#*=}"
      shift
      ;;
    *)
      echo "Unknown option: $arg"
      ;;
  esac
done

cd /app/shared

./"$EXECUTABLE" "$RANDOM_SEED" > "$INPUT_FILE"