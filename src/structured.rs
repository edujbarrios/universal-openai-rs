use serde_json::Value;

pub fn schema_for<T>() -> Value
where
    T: schemars::JsonSchema,
{
    serde_json::to_value(schemars::schema_for!(T)).expect("schemars generated invalid JSON")
}

pub fn schema_name<T>() -> String {
    std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or("response")
        .to_string()
}
