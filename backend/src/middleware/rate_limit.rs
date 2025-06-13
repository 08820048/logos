use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

use axum::{
    extract::ConnectInfo,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    body::Body,
};

/// 限流配置
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// 时间窗口，单位为秒
    pub window_seconds: u64,
    /// 在时间窗口内允许的最大请求数
    pub max_requests: usize,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            window_seconds: 60, // 默认1分钟
            max_requests: 5,    // 默认每分钟5个请求
        }
    }
}

/// 限流中间件状态
#[derive(Debug)]
pub struct RateLimiter {
    /// 请求记录，键为IP地址，值为请求时间列表
    requests: Mutex<HashMap<IpAddr, Vec<Instant>>>,
    /// 限流配置
    config: RateLimitConfig,
}

// 全局静态限流器
static GLOBAL_RATE_LIMITER: OnceLock<RateLimiter> = OnceLock::new();

impl RateLimiter {
    /// 获取全局限流器实例
    pub fn global(config: RateLimitConfig) -> &'static RateLimiter {
        GLOBAL_RATE_LIMITER.get_or_init(|| {
            RateLimiter {
                requests: Mutex::new(HashMap::new()),
                config,
            }
        })
    }

    /// 检查请求是否应该被限流
    pub fn should_limit(&self, ip: &IpAddr) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        let window = Duration::from_secs(self.config.window_seconds);

        // 获取IP的请求记录，如果不存在则创建新的
        let ip_requests = requests.entry(*ip).or_insert_with(Vec::new);

        // 清理过期的请求记录
        ip_requests.retain(|time| now.duration_since(*time) < window);

        // 检查是否超过限制
        if ip_requests.len() >= self.config.max_requests {
            return true;
        }

        // 记录新的请求
        ip_requests.push(now);
        false
    }
}

/// 限流中间件层
#[derive(Clone)]
pub struct RateLimitLayer {
    config: RateLimitConfig,
}

impl RateLimitLayer {
    /// 创建新的限流中间件层
    pub fn new(config: RateLimitConfig) -> Self {
        Self { config }
    }
}

/// 限流中间件处理函数
pub async fn rate_limit(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    req: Request<Body>,
    next: Next<Body>,
    config: RateLimitConfig,
) -> impl IntoResponse {
    let ip = addr.ip();
    let limiter = RateLimiter::global(config);
    
    if limiter.should_limit(&ip) {
        // 如果应该限流，返回429状态码
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    
    // 否则继续处理请求
    next.run(req).await
}
