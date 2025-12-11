use anyhow::{Context, Result};

fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("读取文件失败: {}", path))
}
