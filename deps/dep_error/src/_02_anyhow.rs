use anyhow::{Context, Result};
use wjj_std::BaseFmtErrs::SysFmtErr;
use wjj_std::{FmtErr, RawErr};
use serde_json::json;
use std::result::Result::Err;

fn read_file_with_context(path: &str) -> Result<String> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read {}", path))?;
    Ok(content)
}

fn read_file_without_context(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)?; // `?` 会把 std::io::Error 转换成 anyhow::Error
    Ok(content)
}

#[test]
fn anyhow_with_context_test() {
    let result = read_file_with_context("/");
    println!("{:?}", result);
}

// Err(Failed to read /
//
// Caused by:
// Is a directory (os error 21)
//
// Stack backtrace:
// 0: <E as anyhow::context::ext::StdError>::ext_context
// at /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/backtrace.rs:27:14
// 1: anyhow::context::<impl anyhow::Context<T,E> for core::result::Result<T,E>>::with_context
// at /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs:65:37
// 2: vendor_error::_02_anyhow::read_file
// at ./src/_02_anyhow.rs:5:39
// 3: vendor_error::_02_anyhow::anyhow_with_context_test
// at ./src/_02_anyhow.rs:16:18
// 4: vendor_error::_02_anyhow::anyhow_with_context_test::{{closure}}
#[test]
fn anyhow_without_context_test() {
    let result = read_file_without_context("/");
    println!("{:?}", result);
}
// Err(Is a directory (os error 21)
//
// Stack backtrace:
// 0: anyhow::error::<impl core::convert::From<E> for anyhow::Error>::from
// at /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/backtrace.rs:27:14
// 1: <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
// at /root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:2184:27
// 2: vendor_error::_02_anyhow::read_file_without_context
// at ./src/_02_anyhow.rs:10:19
// 3: vendor_error::_02_anyhow::anyhow_without_context_test
// at ./src/_02_anyhow.rs:37:18
// 4: vendor_error::_02_anyhow::anyhow_without_context_test::{{closure}}

fn fmt_err() -> Result<()> {
    Err(SysFmtErr
        .to_err(json!({
            "cause": "network error"
        }))
        .into())
}

#[test]
fn fmt_err_test() {
    let err = fmt_err().unwrap_err();
    // 2️⃣ 向下转型，拿回原始 FmtErr
    if let Some(fmt_err) = err.downcast_ref::<FmtErr>() {
        println!("err_code = {}", fmt_err.err_code);
        println!("err_msg = {}", fmt_err.to_string());
    } else if let Some(fmt_err) = err.downcast_ref::<RawErr>() {
        println!("err_code = {}", fmt_err.err_code);
        println!("err_msg = {}", fmt_err.to_string());
    } else {
        println!("Unknown error: {:?}", err);
    }
}
