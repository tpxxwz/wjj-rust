mod util;
mod core;

// -------------------- gen_test --------------------
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

// 直接运行测试会不显示println!的 运行时候需要加上 --nocapture
// 或者加环境变量 RUST_TEST_NOCAPTURE=1
#[proc_macro_attribute]
pub fn gen_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    // 保存原始函数名
    let original_name = input.sig.ident.clone();
    // 改名后的函数名
    let new_name = format_ident!("{}_tmp", original_name);
    input.sig.ident = new_name.clone();

    // 测试函数名用原始名字
    // let test_fn_name = original_name.clone();

    let expanded = quote! {
        // 改名后的原函数
        #input

        #[cfg(test)]
        #[test]
        fn #original_name() {
            #new_name(); // ✅ 调用改名后的函数
        }
    };

    TokenStream::from(expanded)
}
