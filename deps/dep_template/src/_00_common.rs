use wjj_std::BaseFmtErrs::SysFmtErr;
use wjj_std::BaseRawErrs::SysRawErr;
use wjj_std::{fmt_err, raw_err};
use serde_json::json;

#[derive(fmt_err)]
#[err_code_prefix = "001"]
enum BizFmtErrs {
    #[error(
        err_code = "00001",
        err_tpl = "user login failed Error cause: {{ cause }}"
    )]
    LoginError,
}

#[derive(raw_err)]
#[err_code_prefix = "001"]
enum BizRawErrs {
    #[error(err_code = "00002", err_msg = "user login limited ")]
    LimitError,
}

#[test]
fn test_hello() {
    let err = BizFmtErrs::LoginError.to_err(json!({
        "cause": "password error"
    }));
    println!("{}", err);
    let err = SysFmtErr.to_err(json!({
        "cause": "network error"
    }));
    println!("{}", err);
    let err = BizRawErrs::LimitError.to_err();
    println!("{}", err);
    let err = SysRawErr.to_err();
    println!("{}", err);
}
