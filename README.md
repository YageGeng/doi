# doi

Rust library for DOI extraction and Crossref metadata retrieval.

## 功能
- 从 URL/文本中提取 DOI（严格匹配 `10.\d+/.+`）
- 解析 DOI 字符串（只保留提取结果）
- 通过 Crossref REST API 获取完整结构化元信息

## 安装
```bash
cargo add doi
```

## 用法
```rust
use doi::{DoiOrgClient, DoiOrgConfig, extract_doi_from_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doi = extract_doi_from_url("https://doi.org/10.5555/12345678")
        .ok_or("doi not found")?;

    let mut config = DoiOrgConfig::default();
    config.user_agent = Some("my-app".to_string());
    config.mailto = Some("me@example.com".to_string());

    let client = DoiOrgClient::new(config)?;
    let response = client.metadata(&doi).await?;

    let title = response
        .title
        .as_ref()
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    println!("DOI: {}", doi.as_str());
    println!("Title: {}", title);
    Ok(())
}
```

## 示例
```bash
cargo run --example basic
```

## 注意事项
- `DoiOrgClient` 会把 `mailto` 放在 `user-agent` 里：`mailto:you@example.com`。
- 如果同时设置 `user_agent` 与 `mailto`，则 header 格式为 `{user_agent} mailto:you@example.com`。
- `CrossrefClient` 使用 `CrossrefConfig` 的 `rate_limit_per_sec`/`concurrency`（`None` 时会根据 `mailto` 自动选择 5/1 或 10/3）。
- 该库只做解析/请求，不做缓存与额外数据源回退。
