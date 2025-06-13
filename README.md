# Logos 博客系统

Logos 是一个轻量、高性能的前后端分离博客系统，后端基于 Rust + MySQL 实现，前端可使用原生 HTML/CSS/JS 或任意前端框架。系统具备完整博客所需的核心功能模块，支持 Markdown 渲染、文章管理、评论、登录、标签、友情链接、全文搜索、RSS 订阅及后台管理。

## 🧱 技术栈

### 后端技术栈

- 语言：Rust
- Web 框架：Axum
- ORM：Sea-ORM
- 数据库：MySQL
- Markdown 渲染：Comrak
- 认证机制：JWT
- JSON 序列化：Serde

## 🧩 功能模块

- 📝 文章：创建/编辑/删除文章、Markdown 支持、分页展示、详情页
- 🏷️ 标签：标签管理、文章打标签、按标签筛选文章
- 🔍 搜索：全文搜索标题和内容
- 💬 评论：公开评论系统，支持邮箱 + 昵称，无需登录
- 🔗 友情链接：展示博客主推荐的外部链接
- 👤 登录认证：管理员登录，操作后台接口需权限验证（JWT）
- 🛠️ 后台管理：管理文章、标签、评论、链接等内容
- 📦 Markdown 渲染：文章内容保存 Markdown，并统一渲染为 HTML 展示
- 📡 RSS 支持：提供 RSS Feed 订阅

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- MySQL 8.0+
- 任意现代浏览器

### 配置

1. 复制环境变量示例文件并修改配置：

```bash
cp backend/.env.example backend/.env
```

2. 编辑 `.env` 文件，设置数据库连接、JWT 密钥和管理员账户等信息。

### 构建与运行

1. 构建后端：

```bash
cd backend
cargo build --release
```

2. 运行后端服务：

```bash
./target/release/logos_blog
```

或直接使用 cargo 运行：

```bash
cargo run --release
```

服务默认在 http://localhost:3000 启动。

## 📚 API 文档

### 认证接口

- `POST /api/login` - 登录，返回 JWT Token

### 文章接口

- `GET /api/posts?page=1&limit=10` - 分页获取文章列表
- `GET /api/posts/{id}` - 获取文章详情
- `POST /api/posts` - 创建文章（需登录）
- `PUT /api/posts/{id}` - 更新文章（需登录）
- `DELETE /api/posts/{id}` - 删除文章（需登录）

### 标签接口

- `GET /api/tags` - 获取所有标签
- `GET /api/tags/{id}/posts` - 获取标签下的文章
- `POST /api/tags` - 创建标签（需登录）
- `DELETE /api/tags/{id}` - 删除标签（需登录）

### 搜索接口

- `GET /api/search?q=关键词` - 搜索文章标题与内容

### 评论接口

- `GET /api/posts/{id}/comments` - 获取文章下的评论
- `POST /api/posts/{id}/comments` - 提交评论（匿名）
- `GET /api/comments` - 获取所有评论（需登录）
- `PUT /api/comments/{id}/status` - 更新评论状态（需登录）
- `DELETE /api/comments/{id}` - 删除评论（需登录）

### 友情链接接口

- `GET /api/links` - 获取所有友情链接
- `POST /api/links` - 添加链接（需登录）
- `PUT /api/links/{id}` - 更新链接（需登录）
- `DELETE /api/links/{id}` - 删除链接（需登录）

### RSS 接口

- `GET /rss.xml` - 获取最新文章的 RSS Feed

## 🧪 测试

运行单元测试和集成测试：

```bash
cargo test
```

## 📝 许可证

MIT License
