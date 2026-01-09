use common_core::BaseFmtErrs::SysFmtErr;
use common_core::BaseRawErrs::SysRawErr;
use common_macros::{FmtErr, RawErr};
use serde_json::json;

#[derive(FmtErr)]
#[err_code_prefix = "001"]
enum BizFmtErrs {
    #[error(
        err_code = "00001",
        err_tpl = "user login failed Error cause: {{ cause }}"
    )]
    LoginError,
}

#[derive(RawErr)]
#[err_code_prefix = "001"]
enum BizRawErrs {
    #[error(err_code = "00002", err_msg = "user login limited ")]
    LimitError,
}

#[test]
fn test_hello() {
    let err = BizFmtErrs::LoginError.fmt(json!({
        "cause": "password error"
    }));
    println!("{}", err);
    let err = SysFmtErr.fmt(json!({
        "cause": "network error"
    }));
    println!("{}", err);
    let err = BizRawErrs::LimitError.raw();
    println!("{}", err);
    let err = SysRawErr.raw();
    println!("{}", err);
}
