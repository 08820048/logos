/// 从 Markdown 文本中提取摘要
pub fn extract_summary(markdown: &str, max_length: usize) -> String {
    // 直接从 Markdown 文本提取摘要，不进行渲染
    let text = markdown.to_string();
    
    let summary = text
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(3)
        .collect::<Vec<&str>>()
        .join(" ");
    
    if summary.len() <= max_length {
        summary
    } else {
        let mut truncated = summary.chars().take(max_length - 3).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}

/// 渲染 Markdown 文本
/// 
/// 注意：在这个博客系统中，我们不在后端渲染 Markdown，而是在前端渲染
/// 这个函数只是为了兼容现有代码，直接返回原始 Markdown 文本
pub fn render_markdown(markdown: &str) -> String {
    // 直接返回原始 Markdown，不进行渲染
    markdown.to_string()
}
