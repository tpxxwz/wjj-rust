use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_derive(FmtErr, attributes(error))]
pub fn derive_fmt_err(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    expand_fmt_err(ast).into()
}

fn expand_fmt_err(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let enum_name = &ast.ident;

    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => {
            return syn::Error::new(ast.span(), "FmtErr only works on enums").to_compile_error();
        }
    };

    let mut match_arms = Vec::new();
    let mut registrations = Vec::new();

    for v in variants {
        let var_name = &v.ident;

        let mut err_code: Option<String> = None;
        let mut err_tpl: Option<String> = None;

        for attr in &v.attrs {
            if attr.path().is_ident("error") {
                // syn 2.x: parse Meta
                let meta = attr.meta.clone();

                if let syn::Meta::List(list) = meta {
                    let args = match list.parse_args_with(
                        syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated
                    ) {
                        Ok(a) => a,
                        Err(e) => return e.to_compile_error(),
                    };

                    for nv in args {
                        let ident = match nv.path.get_ident() {
                            Some(i) => i.to_string(),
                            None => {
                                return syn::Error::new_spanned(
                                    nv.path,
                                    "expected simple identifier",
                                )
                                .to_compile_error();
                            }
                        };

                        // 取字面量字符串
                        let lit_str = match &nv.value {
                            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                                syn::Lit::Str(s) => s,
                                _ => {
                                    return syn::Error::new_spanned(
                                        &expr_lit.lit,
                                        "value must be string literal",
                                    )
                                    .to_compile_error();
                                }
                            },
                            _ => {
                                return syn::Error::new_spanned(
                                    &nv.value,
                                    "value must be literal string",
                                )
                                .to_compile_error();
                            }
                        };
                        let lit_val = lit_str.value();
                        match ident.as_str() {
                            "err_code" => {
                                // 这里加 4 位数字的校验
                                if !is_err_code_numeric_fixed_len(6, &lit_val) {
                                    // 把错误定位在具体的字符串字面量上
                                    let span = lit_str.span();
                                    return syn::Error::new(
                                        span,
                                        "err_code must be exactly 4 digits (e.g., \"1001\")",
                                    )
                                    .to_compile_error();
                                }
                                err_code = Some(lit_val);
                            }
                            "err_tpl" => err_tpl = Some(lit_val),
                            _ => {}
                        }
                    }
                }
            }
        }
        let err_code = match err_code {
            Some(s) => s,
            None => {
                return syn::Error::new(v.span(), "err_code missing").to_compile_error();
            }
        };
        let err_tpl = match err_tpl {
            Some(s) => s,
            None => {
                return syn::Error::new(v.span(), "err_tpl missing").to_compile_error();
            }
        };
        // 生成 match arm
        match_arms.push(quote! {
            #enum_name::#var_name => common_core::FmtErr {
                err_code: #err_code,
                err_args: args.clone(),
            }
        });

        // 生成 inventory 注册
        registrations.push(quote! {
            inventory::submit! {
                common_core::TemplateRegistration {
                    f: |env: &mut minijinja::Environment| {
                        env.add_template(#err_code, #err_tpl).unwrap();
                    }
                }
            }
        });
    }

    quote! {
        impl #enum_name {
            pub fn fmt(&self, args: serde_json::Value) -> common_core::FmtErr {
                match self {
                    #(#match_arms),*
                }
            }
        }
        #(#registrations)*
    }
}

#[proc_macro_derive(RawErr, attributes(error))]
pub fn derive_raw_err(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    expand_raw_err(ast).into()
}

fn expand_raw_err(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let enum_name = &ast.ident;

    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => {
            return syn::Error::new(ast.span(), "RawErr only works on enums").to_compile_error();
        }
    };

    let mut match_arms = Vec::new();

    for v in variants {
        let var_name = &v.ident;

        let mut err_code: Option<String> = None;
        let mut err_msg: Option<String> = None;

        for attr in &v.attrs {
            if attr.path().is_ident("error") {
                // syn 2.x: parse Meta
                let meta = attr.meta.clone();

                if let syn::Meta::List(list) = meta {
                    let args = match list.parse_args_with(
                        syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated
                    ) {
                        Ok(a) => a,
                        Err(e) => return e.to_compile_error(),
                    };

                    for nv in args {
                        let ident = match nv.path.get_ident() {
                            Some(i) => i.to_string(),
                            None => {
                                return syn::Error::new_spanned(
                                    nv.path,
                                    "expected simple identifier",
                                )
                                .to_compile_error();
                            }
                        };

                        // 取字面量字符串
                        let lit_str = match &nv.value {
                            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                                syn::Lit::Str(s) => s,
                                _ => {
                                    return syn::Error::new_spanned(
                                        &expr_lit.lit,
                                        "value must be string literal",
                                    )
                                    .to_compile_error();
                                }
                            },
                            _ => {
                                return syn::Error::new_spanned(
                                    &nv.value,
                                    "value must be literal string",
                                )
                                .to_compile_error();
                            }
                        };
                        let lit_val = lit_str.value();
                        match ident.as_str() {
                            "err_code" => {
                                // 这里加 4 位数字的校验
                                if !is_err_code_numeric_fixed_len(6, &lit_val) {
                                    // 把错误定位在具体的字符串字面量上
                                    let span = lit_str.span();
                                    return syn::Error::new(
                                        span,
                                        "err_code must be exactly 4 digits (e.g., \"1001\")",
                                    )
                                    .to_compile_error();
                                }
                                err_code = Some(lit_val);
                            }
                            "err_msg" => err_msg = Some(lit_val),
                            _ => {}
                        }
                    }
                }
            }
        }

        let err_code = match err_code {
            Some(s) => s,
            None => {
                return syn::Error::new(v.span(), "err_code missing").to_compile_error();
            }
        };
        let err_msg = match err_msg {
            Some(s) => s,
            None => {
                return syn::Error::new(v.span(), "err_msg missing").to_compile_error();
            }
        };

        // 生成 match arm
        match_arms.push(quote! {
            #enum_name::#var_name => common_core::RawErr {
                err_code: #err_code,
                err_msg: #err_msg,
            }
        });
    }

    quote! {
        impl #enum_name {
            pub fn raw(&self) -> common_core::RawErr {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

// 简单的 4 位数字校验函数（避免引入 regex crate）
fn is_err_code_numeric_fixed_len(len: usize, s: &str) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_digit())
}
