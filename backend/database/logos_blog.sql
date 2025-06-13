-- Logos 博客系统数据库初始化脚本
-- 创建于 2025-06-13

-- 创建数据库
CREATE DATABASE IF NOT EXISTS `logos_blog` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE `logos_blog`;

-- 用户表
CREATE TABLE IF NOT EXISTS `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(50) NOT NULL,
  `password_hash` varchar(255) NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_username` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 文章表
CREATE TABLE IF NOT EXISTS `posts` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL,
  `slug` varchar(255) NOT NULL,
  `content_md` text NOT NULL,
  `content_html` text NOT NULL,
  `summary` varchar(500) NOT NULL,
  `published` tinyint(1) NOT NULL DEFAULT '0',
  `published_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_slug` (`slug`),
  KEY `idx_published` (`published`),
  KEY `idx_published_at` (`published_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 标签表
CREATE TABLE IF NOT EXISTS `tags` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(50) NOT NULL,
  `slug` varchar(50) NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_name` (`name`),
  UNIQUE KEY `idx_slug` (`slug`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 文章标签关联表
CREATE TABLE IF NOT EXISTS `post_tags` (
  `post_id` int(11) NOT NULL,
  `tag_id` int(11) NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`post_id`,`tag_id`),
  KEY `fk_post_tags_tag_id` (`tag_id`),
  CONSTRAINT `fk_post_tags_post_id` FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_post_tags_tag_id` FOREIGN KEY (`tag_id`) REFERENCES `tags` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 评论表
CREATE TABLE IF NOT EXISTS `comments` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `post_id` int(11) NOT NULL,
  `nickname` varchar(50) NOT NULL,
  `email` varchar(100) NOT NULL,
  `content` text NOT NULL,
  `status` varchar(20) NOT NULL DEFAULT 'pending',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `fk_comments_post_id` (`post_id`),
  KEY `idx_status` (`status`),
  CONSTRAINT `fk_comments_post_id` FOREIGN KEY (`post_id`) REFERENCES `posts` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 友情链接表
CREATE TABLE IF NOT EXISTS `links` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(50) NOT NULL,
  `url` varchar(255) NOT NULL,
  `description` varchar(255) DEFAULT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- 创建全文索引
ALTER TABLE `posts` ADD FULLTEXT INDEX `idx_fulltext` (`title`, `content_md`);

-- 插入默认管理员用户（密码为 admin，实际使用时请修改）
-- 注意：这里的密码哈希是使用 Argon2 生成的，实际部署时应该替换为安全的密码
INSERT INTO `users` (`username`, `password_hash`) VALUES 
('admin', '$argon2id$v=19$m=19456,t=2,p=1$U2FsdFNhbHRTYWx0$mZxJEy1Q7R3+H4JGzKCN8YZA0x6NxqEfvK/Td53zGcQ');

-- 插入示例标签
INSERT INTO `tags` (`name`, `slug`) VALUES 
('Rust', 'rust'),
('Web开发', 'web-development'),
('教程', 'tutorial');

-- 插入示例文章
INSERT INTO `posts` (`title`, `slug`, `content_md`, `content_html`, `summary`, `published`, `published_at`) VALUES 
('欢迎使用 Logos 博客系统', 'welcome-to-logos', '# 欢迎使用 Logos 博客系统\n\n这是一个基于 Rust 开发的高性能博客系统。\n\n## 特点\n\n- 高性能\n- Markdown 支持\n- 标签系统\n- 评论功能\n- 全文搜索\n\n```rust\nfn main() {\n    println!("Hello, Logos!");\n}\n```', '<h1>欢迎使用 Logos 博客系统</h1>\n<p>这是一个基于 Rust 开发的高性能博客系统。</p>\n<h2>特点</h2>\n<ul>\n<li>高性能</li>\n<li>Markdown 支持</li>\n<li>标签系统</li>\n<li>评论功能</li>\n<li>全文搜索</li>\n</ul>\n<pre><code class="language-rust">fn main() {\n    println!("Hello, Logos!");\n}\n</code></pre>', '这是一个基于 Rust 开发的高性能博客系统，支持 Markdown、标签、评论、全文搜索等功能。', 1, NOW());

-- 关联文章和标签
INSERT INTO `post_tags` (`post_id`, `tag_id`) VALUES 
(1, 1),
(1, 2);

-- 添加示例评论
INSERT INTO `comments` (`post_id`, `nickname`, `email`, `content`, `status`) VALUES 
(1, '访客', 'guest@example.com', '这个博客系统看起来很不错！', 'approved');

-- 添加示例友情链接
INSERT INTO `links` (`name`, `url`, `description`) VALUES 
('Rust 官网', 'https://www.rust-lang.org/', 'Rust 编程语言官方网站');
