use std::collections::HashSet;

/// 将标题转换为 URL 友好的 slug
pub fn slugify(text: &str) -> String {
    // 转换为小写
    let text = text.to_lowercase();
    
    // 将非字母数字字符替换为连字符
    let mut result = String::new();
    let mut prev_is_separator = true;
    
    for c in text.chars() {
        if c.is_alphanumeric() {
            result.push(c);
            prev_is_separator = false;
        } else if !prev_is_separator {
            result.push('-');
            prev_is_separator = true;
        }
    }
    
    // 移除开头和结尾的连字符
    let result = result.trim_matches('-').to_string();
    
    // 如果为空，返回一个默认值
    if result.is_empty() {
        return "post".to_string();
    }
    
    result
}

/// 确保 slug 在集合中唯一，如果不唯一则添加数字后缀
pub fn ensure_unique_slug(slug: &str, existing_slugs: &HashSet<String>) -> String {
    if !existing_slugs.contains(slug) {
        return slug.to_string();
    }
    
    let mut counter = 1;
    loop {
        let new_slug = format!("{}-{}", slug, counter);
        if !existing_slugs.contains(&new_slug) {
            return new_slug;
        }
        counter += 1;
    }
}
