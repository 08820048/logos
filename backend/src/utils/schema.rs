use utoipa::ToSchema;

// 使用自定义结构体来为 DateTimeWithTimeZone 提供 schema
#[derive(ToSchema)]
#[schema(schema_type = String, format = "date-time")]
pub struct DateTimeSchema;

// 注册自定义类型的函数
pub fn register_schema_components() {}

