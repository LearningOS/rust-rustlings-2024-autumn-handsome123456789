//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs


use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
     // 启用 "pass" feature，用于 tests8
     println!("cargo:rustc-cfg=feature=\"pass\"");

    // 获取当前的 Unix 时间戳（秒为单位）
    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("系统时间有问题，无法获取当前时间"),
    };

    // 使用 `cargo:rustc-env` 告诉 Cargo 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}