#!/bin/bash
set -e

LANG=$1
cd /app/shared

echo "언어: $LANG"

case "$LANG" in
  "c99")
    gcc Main.c -o Main -O2 -Wall -lm -static -std=gnu99 &&
    ./Main
    ;;
  "c++17")
    g++ Main.cc -o Main -O2 -Wall -lm -static -std=gnu++17 -DONLINE_JUDGE -DBOJ &&
    ./Main
    ;;
  "c++20")
    g++ Main.cc -o Main -O2 -Wall -lm -static -std=gnu++20 -DONLINE_JUDGE -DBOJ &&
    ./Main
    ;;
  "java8")
    javac -J-Xms1024m -J-Xmx1920m -J-Xss512m -encoding UTF-8 Main.java &&
    java -Xms1024m -Xmx1920m -Xss512m -Dfile.encoding=UTF-8 Main
    ;;
  "python3")
    python3 -W ignore -c "import py_compile; py_compile.compile(r'Main.py')" &&
    python3 -W ignore Main.py
    ;;
  "pypy")
    pypy3 -W ignore -c "import py_compile; py_compile.compile(r'Main.py')" &&
    pypy3 -W ignore Main.py
    ;;
  *)
    echo "지원하지 않는 언어: $LANG"
    exit 1
    ;;
esac
