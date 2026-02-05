# doi

Rust library for DOI extraction and Crossref metadata retrieval.

## 功能
- 从 URL/文本中提取 DOI（严格匹配 `10.\d+/.+`）
- 规范化 DOI（保留原始值 + canonical）
- 通过 Crossref REST API 获取完整结构化元信息

## 安装
```bash
cargo add doi
```

## 用法
```rust
use doi::{CrossrefClient, CrossrefConfig, extract_doi_from_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doi = extract_doi_from_url("https://doi.org/10.5555/12345678")?;

    let mut config = CrossrefConfig::default();
    config.user_agent = Some("my-app".to_string());

    let client = CrossrefClient::new(config)?;
    let response = client.fetch_metadata(&doi).await?;

    let title = response.message.title.first().cloned().unwrap_or_default();
    println!("DOI: {}", doi.canonical);
    println!("Title: {}", title);
    Ok(())
}
```

## 示例
```bash
cargo run --example basic
```

## 注意事项
- 请求会自动带 `mailto=icoderdev@outlook.com`（可在 config 中覆盖）。
- `user_agent` 只有在提供时才会设置，格式为 `{app_name} {mailto}`。
- 该库只做解析/请求，不做缓存与额外数据源回退。
