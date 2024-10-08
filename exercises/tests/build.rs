//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs


use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 检查是否定义了 `pass` 特性
    if let Some(_) = env::var("CARGO_FEATURE_PASS").ok() {
        // 如果定义了 `pass` 特性，设置一个特殊的 cfg 标志
        println!("cargo:rustc-cfg=pass");
        return;
    }

    // 获取当前的 Unix 时间戳（秒为单位）
    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("系统时间有问题，无法获取当前时间"),
    };

    // 使用 `cargo:rustc-env` 告诉 Cargo 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}