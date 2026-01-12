use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_derive(fmt_err, attributes(err_code_prefix, error))]
pub fn derive_fmt_err(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let wrap_all = &mut WrapAll {
        match_arms: Vec::new(),
        template_registrations: Vec::new(),
        err_code_registrations: Vec::new(),
        tpl_reg_seq: 0,
    };
    expand(ast, "fmt_err", wrap_all, Wrap::new).into()
}

#[proc_macro_derive(raw_err, attributes(err_code_prefix, error))]
pub fn derive_raw_err(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let wrap_all = &mut WrapAll1 {
        match_arms: Vec::new(),
        err_code_registrations: Vec::new(),
    };
    expand(ast, "raw_err", wrap_all, Wrap1::new).into()
}

trait WrapAllInterface<Wrap: WrapInterface> {
    fn push(&mut self, enum_name: &syn::Ident, wrap: Wrap);
    fn quote(&mut self, enum_name: &syn::Ident) -> proc_macro2::TokenStream;
}

trait WrapInterface {
    fn new(var_name: syn::Ident) -> Self;
    fn update(&mut self, ident: String, lit_val: String);
    fn check(&mut self, err_code_prefix: &String) -> Option<proc_macro2::TokenStream>;
    fn get_err_code_var_name(&mut self) -> (&mut Option<String>, &syn::Ident);
    fn check_err_code(&mut self, err_code_prefix: &String) -> Option<proc_macro2::TokenStream> {
        let (err_code, var_name) = self.get_err_code_var_name();
        if let Some(code) = err_code.as_mut() {
            if !ensure_err_code_numeric_len(code, 5) {
                return Some(expand_err(
                    var_name.span(),
                    "err_code must be exactly 5 digits",
                ));
            }
            *code = format!("{}{}", err_code_prefix, code);
            None
        } else {
            Some(expand_err_span(var_name, "err_code missing"))
        }
    }
}

struct WrapAll {
    match_arms: Vec<proc_macro2::TokenStream>,
    template_registrations: Vec<proc_macro2::TokenStream>,
    err_code_registrations: Vec<proc_macro2::TokenStream>,
    tpl_reg_seq: usize,
}

impl WrapAllInterface<Wrap> for WrapAll {
    fn push(&mut self, enum_name: &syn::Ident, wrap: Wrap) {
        let err_code = wrap.err_code;
        let err_tpl = wrap.err_tpl;
        let var_name = wrap.var_name;
        self.match_arms.push(quote! {
            #enum_name::#var_name => common_core::FmtErr {
                err_code: #err_code,
                err_tpl: #err_tpl,
                err_args: args.clone(),
            }
        });
        let err_code_reg = format_ident!("ERR_CODE_{}", err_code.clone().unwrap_or("".to_string()));
        self.err_code_registrations.push(quote! {
            #[linkme::distributed_slice(common_core::ERR_CODE_REGISTRATIONS)]
            static #err_code_reg: common_core::ErrCodeRegistration =
                common_core::ErrCodeRegistration {
                    err_code: #err_code,
                };
        });
        self.tpl_reg_seq += 1;
        let tpl_reg = format_ident!("TPL_REG_SEQ_{}", self.tpl_reg_seq);
        let tpl_reg_f = format_ident!("TPL_REG_SEQ_{}_f", self.tpl_reg_seq);
        let enum_name_str = enum_name.to_string();
        let var_name_str = var_name.to_string();
        self.template_registrations.push(quote! {
            fn #tpl_reg_f(env: &mut common_core::Environment<'static>) {
                env.add_template(#err_code, #err_tpl).expect(&format!(
                    "template registration failed, enum: {}, var_name: {}, err_code: {}, err_tpl: {}",
                    #enum_name_str, #var_name_str, #err_code, #err_tpl)
                );
            }
            #[linkme::distributed_slice(common_core::TEMPLATE_REGISTRATIONS)]
            static #tpl_reg: common_core::TemplateRegistration =
                common_core::TemplateRegistration {
                    f: #tpl_reg_f,
                };
        });
    }
    fn quote(&mut self, enum_name: &syn::Ident) -> proc_macro2::TokenStream {
        let match_arms = self.match_arms.clone();
        let err_code_registrations = self.err_code_registrations.clone();
        let template_registrations = self.template_registrations.clone();
        quote! {
            impl #enum_name {
                pub fn to_err(&self, args: common_core::Value) -> common_core::FmtErr {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
            #(#err_code_registrations)*
            #(#template_registrations)*
        }
    }
}

struct Wrap {
    err_code: Option<String>,
    err_tpl: Option<String>,
    var_name: syn::Ident,
}

impl WrapInterface for Wrap {
    fn new(var_name: syn::Ident) -> Wrap {
        Wrap {
            err_code: None,
            err_tpl: None,
            var_name,
        }
    }
    fn update(&mut self, ident: String, lit_val: String) {
        match ident.as_str() {
            "err_code" => self.err_code = Some(lit_val),
            "err_tpl" => self.err_tpl = Some(lit_val),
            _ => {}
        }
    }
    fn check(&mut self, err_code_prefix: &String) -> Option<proc_macro2::TokenStream> {
        if let Some(err) = self.check_err_code(err_code_prefix) {
            return Some(err);
        }
        if self.err_tpl.is_none() {
            return Some(expand_err_span(self.var_name.clone(), "err_tpl missing"));
        }
        None
    }
    fn get_err_code_var_name(&mut self) -> (&mut Option<String>, &syn::Ident) {
        (&mut self.err_code, &self.var_name)
    }
}

struct WrapAll1 {
    match_arms: Vec<proc_macro2::TokenStream>,
    err_code_registrations: Vec<proc_macro2::TokenStream>,
}
impl WrapAllInterface<Wrap1> for WrapAll1 {
    fn push(&mut self, enum_name: &syn::Ident, wrap: Wrap1) {
        let err_code = wrap.err_code;
        let err_msg = wrap.err_msg;
        let var_name = wrap.var_name;
        self.match_arms.push(quote! {
            #enum_name::#var_name => common_core::RawErr {
                err_code: #err_code,
                err_msg: #err_msg,
            }
        });
        let err_code_reg = format_ident!("ERR_CODE_{}", err_code.clone().unwrap_or("".to_string()));
        self.err_code_registrations.push(quote! {
            #[linkme::distributed_slice(common_core::ERR_CODE_REGISTRATIONS)]
            static #err_code_reg: common_core::ErrCodeRegistration =
                common_core::ErrCodeRegistration {
                    err_code: #err_code,
                };
        });
    }
    fn quote(&mut self, enum_name: &syn::Ident) -> proc_macro2::TokenStream {
        let match_arms = self.match_arms.clone();
        let err_code_registrations = self.err_code_registrations.clone();
        quote! {
            impl #enum_name {
                pub fn to_err(&self) -> common_core::RawErr {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
            #(#err_code_registrations)*
        }
    }
}

struct Wrap1 {
    err_code: Option<String>,
    err_msg: Option<String>,
    var_name: syn::Ident,
}

impl WrapInterface for Wrap1 {
    fn new(var_name: syn::Ident) -> Wrap1 {
        Wrap1 {
            err_code: None,
            err_msg: None,
            var_name,
        }
    }
    fn update(&mut self, ident: String, lit_val: String) {
        match ident.as_str() {
            "err_code" => self.err_code = Some(lit_val),
            "err_msg" => self.err_msg = Some(lit_val),
            _ => {}
        }
    }
    fn check(&mut self, err_code_prefix: &String) -> Option<proc_macro2::TokenStream> {
        if let Some(err) = self.check_err_code(err_code_prefix) {
            return Some(err);
        }
        if self.err_msg.is_none() {
            return Some(expand_err_span(self.var_name.clone(), "err_msg missing"));
        }
        None
    }
    fn get_err_code_var_name(&mut self) -> (&mut Option<String>, &syn::Ident) {
        (&mut self.err_code, &self.var_name)
    }
}

fn expand<WrapAll, Wrap, F>(
    ast: syn::DeriveInput,
    derive_name: &str,
    wrap_all: &mut WrapAll,
    wrap_new_fn: F,
) -> proc_macro2::TokenStream
where
    Wrap: WrapInterface,
    WrapAll: WrapAllInterface<Wrap>,
    F: Fn(syn::Ident) -> Wrap,
{
    let enum_name = &ast.ident;
    let variants = match ast.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => {
            return expand_err_span(ast, format!("{} only works on enums", derive_name).as_str());
        }
    };
    // -------------------------
    // 校验属性贴错位置
    // 解析 enum 上的 err_code_prefix
    // -------------------------
    let mut err_code_prefix: Option<String> = None;
    for attr in &ast.attrs {
        // error 不能贴在 enum 上
        if attr.path().is_ident("error") {
            return expand_err_span(attr, "`error` attribute is only allowed on enum variants");
        }
        if attr.path().is_ident("err_code_prefix") {
            // 必须是 #[err_code_prefix = "..."]
            let meta = &attr.meta;
            let nv = match meta {
                syn::Meta::NameValue(nv) => nv,
                _ => {
                    return expand_err_span(
                        attr,
                        "`err_code_prefix` must be written as #[err_code_prefix = \"0001\"]",
                    );
                }
            };
            // 必须是字符串字面量
            let lit = match &nv.value {
                syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                    syn::Lit::Str(s) => s,
                    _ => {
                        return expand_err_span(
                            &expr_lit.lit,
                            "`err_code_prefix` value must be a string literal, e.g. \"0001\"",
                        );
                    }
                },
                _ => {
                    return expand_err_span(
                        &nv.value,
                        "`err_code_prefix` must be a string literal like \"0001\" (e.g. #[err_code_prefix = \"0001\"])",
                    );
                }
            };
            let prefix = lit.value();
            if !ensure_err_code_numeric_len(&prefix, 3) {
                return expand_err(
                    lit.span(),
                    "`err_code_prefix` must be exactly 3 digits, e.g. \"001\"",
                );
            }
            err_code_prefix = Some(prefix);
        }
    }

    // -------------------------
    // 强制要求 err_code_prefix 必须存在
    // -------------------------
    let err_code_prefix = match err_code_prefix {
        Some(prefix) => prefix,
        None => {
            return expand_err_span(
                ast.ident,
                "`err_code_prefix` attribute is required on the enum",
            );
        }
    };
    for variant in variants {
        let var_name = &variant.ident;
        let mut wrap = wrap_new_fn(var_name.clone());
        for attr in &variant.attrs {
            // err_code_prefix 不能贴在 enum variants 上
            if attr.path().is_ident("err_code_prefix") {
                return expand_err_span(
                    attr,
                    "`err_code_prefix` attribute is only allowed on the enum, not on variants",
                );
            }
            if attr.path().is_ident("error") {
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
                                return expand_err_span(
                                    &nv.path,
                                    "expected identifier like `err_code = \"0001\"`",
                                );
                            }
                        };

                        let lit_str = match &nv.value {
                            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                                syn::Lit::Str(s) => s,
                                _ => {
                                    return expand_err_span(
                                        &expr_lit.lit,
                                        "value must be string literal",
                                    );
                                }
                            },
                            _ => return expand_err_span(&nv.value, "value must be literal string"),
                        };
                        let lit_val = lit_str.value();
                        wrap.update(ident, lit_val);
                    }
                }
            }
        }
        if let Some(err) = wrap.check(&err_code_prefix) {
            return err;
        }
        wrap_all.push(enum_name, wrap)
    }

    wrap_all.quote(enum_name)
}

// 简单的 len 位数字校验函数（避免引入 regex crate）
fn ensure_err_code_numeric_len(code: &String, len: usize) -> bool {
    code.len() == len && code.bytes().all(|b| b.is_ascii_digit())
}

// ✔ 结构错误 → 用 `new_spanned(tokens)`
// 例如：
// 1. 属性类型不对
// 2. 属性放错位置
// 3. 属性参数结构不对
// 4. 字段或 variant 缺失
// 5. AST 节点类型不符
// 6. 宏语法结构错误
// 7. syn 解析失败
fn expand_err_span(token: impl quote::ToTokens, msg: &str) -> proc_macro2::TokenStream {
    syn::Error::new_spanned(token, msg).to_compile_error()
}

// ✔ 值错误 → 用 `new(span)`
// 例如：
// 1. 字符串格式不对
// 2. 字符串长度不对
// 3. 数字范围不对
// 4. 标识符非法
// 5. 不允许的路径值
// 6. 不允许的类型
// 7. 重复定义
// 8. 不允许的组合
fn expand_err(span: proc_macro2::Span, msg: &str) -> proc_macro2::TokenStream {
    syn::Error::new(span, msg).to_compile_error()
}
