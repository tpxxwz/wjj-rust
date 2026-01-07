use crate::_01_thiserror::AppError;
use std::io;

fn anyhow_with_this_error() -> anyhow::Result<()> {
    let err = io::Error::new(io::ErrorKind::NotFound, "数据库连接失败");
    let app_error: AppError = err.into();
    Err(app_error.into())
}

#[test]
fn anyhow_with_this_error_test() {
    anyhow_with_this_error().unwrap();
}
