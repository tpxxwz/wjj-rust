use jaq_parse::parse;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_value};
use std::collections::BTreeMap;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    #[serde(rename = "cityName")]
    city: Option<String>, // 允许 null
    zip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    #[serde(rename = "fullName")]
    name: String,
    age: Option<u32>,         // 允许 null
    address: Option<Address>, // 整个嵌套结构体可以是 null
}

#[test]
fn struct_to_json() {
    let user = User {
        name: "Alice".to_string(),
        age: None,     // null
        address: None, // null
    };

    // 普通打印
    let json_str = serde_json::to_string(&user).unwrap();
    println!("JSON:\n{}", json_str);

    // 如果想格式化打印，可以用：
    // let json_str = serde_json::to_string_pretty(&user).unwrap();
    // println!("JSON:\n{}", json_str);
}

#[test]
fn json_to_struct() {
    let json_str = r#"
    {
        "fullName": "Bob",
        "age": null,
        "address": {
            "cityName": null,
            "zip": "530-0001"
        }
    }
    "#;

    let user: User = serde_json::from_str(json_str).unwrap();
    println!("Struct: {:?}", user);
}

#[test]
fn write_json_to_file() {
    let user = User {
        name: "Alice".to_string(),
        age: None, // null
        address: Some(Address {
            city: Some("Tokyo".to_string()),
            zip: "100-0001".to_string(),
        }),
    };

    let file = File::create("user.json").unwrap();
    serde_json::to_writer_pretty(file, &user).unwrap();
    println!("✅ JSON 已写入 user.json");
}
#[test]
fn read_json_from_file() {
    let file = File::open("user.json").unwrap();
    let user: User = serde_json::from_reader(file).unwrap();
    println!("✅ 从文件读取的结构体: {:?}", user);
}

// 定义一个泛型结构体
#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

// 第一个结构体：产品信息
#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

// 第二个结构体：订单信息
#[derive(Serialize, Deserialize, Debug)]
struct Order {
    order_id: String,
    product_ids: Vec<u32>,
    total_amount: f64,
}

#[test]
fn generic_struct_to_json() {
    // ✅ 使用 Product
    let product_response = ApiResponse {
        status: "success".to_string(),
        data: Product {
            id: 101,
            name: "Laptop".to_string(),
            price: 1299.99,
        },
    };

    let product_json = serde_json::to_string_pretty(&product_response).unwrap();
    println!("Product JSON:\n{}\n", product_json);

    // ✅ 使用 Order
    let order_response = ApiResponse {
        status: "success".to_string(),
        data: Order {
            order_id: "ORD20251210".to_string(),
            product_ids: vec![101, 102, 103],
            total_amount: 2999.50,
        },
    };

    let order_json = serde_json::to_string_pretty(&order_response).unwrap();
    println!("Order JSON:\n{}\n", order_json);
}

#[test]
fn generic_json_to_struct() {
    // ✅ JSON → Struct（解析 Product）
    let json_input_product = r#"
    {
        "status": "success",
        "data": {
            "id": 202,
            "name": "Smartphone",
            "price": 899.99
        }
    }
    "#;
    let parsed_product: ApiResponse<Product> = serde_json::from_str(json_input_product).unwrap();
    println!("Parsed Product Struct: {:?}\n", parsed_product);

    // ✅ JSON → Struct（解析 Order）
    let json_input_order = r#"
    {
        "status": "success",
        "data": {
            "order_id": "ORD20251211",
            "product_ids": [202, 203],
            "total_amount": 1799.98
        }
    }
    "#;
    let parsed_order: ApiResponse<Order> = serde_json::from_str(json_input_order).unwrap();
    println!("Parsed Order Struct: {:?}", parsed_order);
}

#[test]
fn generic_json_to_dynamic() {
    let json_input = r#"
    {
        "status": "success",
        "data": {
            "id": 202,
            "name": "Smartphone",
            "price": 899.99
        }
    }
    "#;

    // 用 Value 作为泛型类型
    let parsed: ApiResponse<Value> = serde_json::from_str(json_input).unwrap();
    // let parsed: ApiResponse<BTreeMap<String, Value>> = serde_json::from_str(json_input).unwrap();
    println!("Parsed dynamic: {:?}", parsed);

    // 访问字段
    if let Some(name) = parsed.data.get("name") {
        println!("Name: {}", name);
    }
    // value 直接变成struct
    let product: Product = from_value(parsed.data).unwrap();
    println!("✅ 转换后的结构体: {:?}", product);
}

// 复杂对象：博客文章
#[derive(Serialize, Deserialize, Debug)]
struct BlogPost {
    title: String,
    author: Author,
    tags: Vec<String>,
    comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    user: String,
    content: String,
    likes: u32,
}
#[test]
fn jq_json_path() {
    // 模拟 JSON 输入
    let json_input = r#"
    {
        "status": "success",
        "data": {
            "title": "Rust Generics Explained",
            "author": {
                "name": "Alice",
                "email": "alice@example.com"
            },
            "tags": ["rust", "programming", "generics"],
            "comments": [
                { "user": "Bob", "content": "Great post!", "likes": 10 },
                { "user": "Charlie", "content": "Very helpful!", "likes": 5 }
            ]
        }
    }
    "#;

    // 1. 用 serde_json 解析成 Value
    let value: Value = serde_json::from_str(json_input).unwrap();

    // 2. 编译 jq 表达式：查询所有评论的点赞数
    let src = ".data.comments[].likes";
    let (maybe_filter, errors) = parse(src, jaq_parse::main());

    if !errors.is_empty() {
        println!("解析过程中有错误: {:?}", errors);
    }

    if let Some(filter) = maybe_filter {
        println!("✅ 解析成功: {:?}", filter);
    } else {
        println!("❌ 解析失败");
    }
}
