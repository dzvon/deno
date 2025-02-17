// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

use std::process::Command;
use test_util::deno_cmd;

#[cfg(debug_assertions)]
const BUILD_VARIANT: &str = "debug";

#[cfg(not(debug_assertions))]
const BUILD_VARIANT: &str = "release";

#[test]
fn basic() {
  let mut build_plugin_base = Command::new("cargo");
  let mut build_plugin =
    build_plugin_base.arg("build").arg("-p").arg("test_ffi");
  if BUILD_VARIANT == "release" {
    build_plugin = build_plugin.arg("--release");
  }
  let build_plugin_output = build_plugin.output().unwrap();
  assert!(build_plugin_output.status.success());
  let output = deno_cmd()
    .arg("run")
    .arg("--allow-ffi")
    .arg("--allow-read")
    .arg("--unstable")
    .arg("tests/test.js")
    .env("NO_COLOR", "1")
    .output()
    .unwrap();
  let stdout = std::str::from_utf8(&output.stdout).unwrap();
  let stderr = std::str::from_utf8(&output.stderr).unwrap();
  if !output.status.success() {
    println!("stdout {}", stdout);
    println!("stderr {}", stderr);
  }
  println!("{:?}", output.status);
  assert!(output.status.success());
  let expected = "\
    dlopen doesn't panic\n\
    something\n\
    [1, 2, 3, 4, 5, 6, 7, 8]\n\
    [1, 2, 3, 4, 5, 6, 7, 8] [9, 10]\n\
    [1, 2, 3, 4, 5, 6, 7, 8]\n\
    [ 1, 2, 3, 4, 5, 6 ]\n\
    [ 4, 5, 6 ]\n\
    [ 4, 5, 6 ]\n\
    Hello from pointer!\n\
    pointer!\n\
    false\n\
    true\n\
    false\n\
    579\n\
    579\n\
    579\n\
    579\n\
    579\n\
    579\n\
    579.9119873046875\n\
    579.912\n\
    Before\n\
    true\n\
    After\n\
    true\n\
    Correct number of resources\n";
  assert_eq!(stdout, expected);
  assert_eq!(stderr, "");
}
