#!/bin/sh

cargo run demo/demo.json | tee demo.dot &&
  dot -Tsvg demo.dot >demo.svg &&
  mv demo.dot demo.svg demo
