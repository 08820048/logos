#[cfg(test)]
mod tests {
    use super::super::{
        markdown::render_markdown,
        password::{hash_password, verify_password},
        slug::slugify,
    };

    #[test]
    fn test_markdown_render() {
        let markdown = "# Hello World\n\nThis is a **test**";
        let html = render_markdown(markdown);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello World"));
        assert!(html.contains("<strong>test</strong>"));
    }

    #[tokio::test]
    async fn test_password_hash_verify() {
        let password = "test_password";
        let hash = hash_password(password).unwrap();
        
        // 验证密码是否匹配
        let result = verify_password(password, &hash).unwrap();
        assert!(result);
        
        // 验证错误密码不匹配
        let result = verify_password("wrong_password", &hash).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("测试文章"), "测试文章");
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("   spaces   "), "spaces");
        assert_eq!(slugify(""), "post");
    }
}
