use rquickjs::FromJs;
use itertools::Itertools;

// copy pasted from https://github.com/DelSkayn/rquickjs/issues/47

pub struct JsonValue(pub serde_json::Value);

impl<'js> FromJs<'js> for JsonValue {
    fn from_js(ctx: rquickjs::Ctx<'js>, v: rquickjs::Value<'js>) -> rquickjs::Result<Self> {
        rquickjs::Result::Ok(JsonValue(match v.type_of() {
            rquickjs::Type::Uninitialized => serde_json::json!("undefined"),
            rquickjs::Type::Undefined => serde_json::json!("undefined"),
            rquickjs::Type::Null => serde_json::json!("null"),
            rquickjs::Type::Bool => serde_json::json!(v.as_bool().unwrap_or(false)),
            rquickjs::Type::Int => serde_json::json!(v.as_int().unwrap_or(0)),
            rquickjs::Type::Float => serde_json::json!(v.as_float().unwrap_or(0.0)),
            rquickjs::Type::String => {
                serde_json::json!(v
                    .as_string()
                    .unwrap_or(&rquickjs::String::from_str(ctx, "")?)
                    .to_string()
                    .unwrap_or(String::from("")))
            }
            rquickjs::Type::Symbol => serde_json::json!("symbol"),
            rquickjs::Type::Array => {
                let empty = &rquickjs::Array::new(ctx)?;
                serde_json::Value::Array(
                    v.as_array()
                        .unwrap_or(empty)
                        .iter::<rquickjs::Value>()
                        .fold_ok(vec![], |mut acc, next| {
                            acc.push(next);
                            acc
                        })?
                        .iter()
                        .map(|v| JsonValue::from_js(ctx, v.clone()).into())
                        .fold_ok(Vec::<serde_json::Value>::new(), |mut acc, next| {
                            acc.push(next.into());
                            acc
                        })?,
                )
            }
            rquickjs::Type::Function => serde_json::json!("function"),
            rquickjs::Type::Object => {
                let mut value = serde_json::Map::<String, serde_json::Value>::new();
                let inner = rquickjs::Object::new(ctx)?;
                let object = v.as_object().unwrap_or(&inner);
                let keys = object.keys::<String>().fold_ok(vec![], |mut acc, next| {
                    acc.push(next);
                    acc
                })?;
                let values = keys
                    .iter()
                    .map(
                        |key| match object.get::<String, rquickjs::Value>(key.clone()) {
                            Ok(value) => Ok((key, value)),
                            Err(err) => Err(err),
                        },
                    )
                    .fold_ok(vec![], |mut acc, next| {
                        acc.push(next);
                        acc
                    })?;
                for (k, v) in values {
                    value.insert(k.clone(), JsonValue::from_js(ctx, v)?.into());
                }
                serde_json::Value::Object(value)
            }
            rquickjs::Type::Module => serde_json::json!("module"),
            rquickjs::Type::Unknown => serde_json::json!("unknown"),
        }))
    }
}

impl Into<serde_json::Value> for JsonValue {
    fn into(self) -> serde_json::Value {
        self.0
    }
}
