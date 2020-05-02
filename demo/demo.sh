#!/bin/sh

cargo run | tee demo.dot &&
  dot -Tsvg demo.dot >demo.svg &&
  mv demo.dot demo.svg demo
