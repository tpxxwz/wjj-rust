use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct GatewayConfig {
    gateway: Gateway,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gateway {
    routes: Vec<Route>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Route {
    id: String,
    uri: String,
    predicates: Vec<String>,
    filters: Vec<String>,
}

#[test]
fn serde_yaml_demo() -> Result<(), Box<dyn std::error::Error>> {
    // ✅ 1. 创建默认配置，包含多个路由
    let default_config = GatewayConfig {
        gateway: Gateway {
            routes: vec![
                Route {
                    id: "route1".to_string(),
                    uri: "http://service1.local".to_string(),
                    predicates: vec!["Path=/service1/**".to_string(), "Method=GET".to_string()],
                    filters: vec!["AddRequestHeader=X-Request-Id,12345".to_string()],
                },
                Route {
                    id: "route2".to_string(),
                    uri: "http://service2.local".to_string(),
                    predicates: vec!["Path=/service2/**".to_string()],
                    filters: vec!["AddResponseHeader=X-Response-Time,100ms".to_string()],
                },
                Route {
                    id: "route3".to_string(),
                    uri: "http://service3.local".to_string(),
                    predicates: vec!["Path=/service3/**".to_string()],
                    filters: vec!["AddRequestHeader=X-New-Header,abc".to_string()],
                },
            ],
        },
    };

    // ✅ 2. 序列化为 YAML 并写入文件
    let yaml_str = serde_yaml::to_string(&default_config)?;
    fs::write("gateway.yaml", yaml_str)?;
    println!("✅ 已写入多个路由到 gateway.yaml");

    // ✅ 3. 再读取文件并解析
    let read_yaml = fs::read_to_string("gateway.yaml")?;
    let config: GatewayConfig = serde_yaml::from_str(&read_yaml)?;
    println!("\n读取后的配置:\n{:#?}", config);

    Ok(())
}
