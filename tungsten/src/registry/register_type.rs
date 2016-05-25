
use toml::Value;

pub trait RegistryType: Sized {
    fn to_value(self) -> Value;
    fn from_value(v: &Value) -> Option<Self>;
}


impl RegistryType for u64 {
    fn to_value(self) -> Value {
        Value::Integer(self as i64)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_integer().map(|e| e as u64)
    }
}

impl RegistryType for i64 {
    fn to_value(self) -> Value {
        Value::Integer(self)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_integer()
    }
}

impl RegistryType for u32 {
    fn to_value(self) -> Value {
        Value::Integer(self as i64)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_integer().map(|e| e as u32)
    }
}

impl RegistryType for i32 {
    fn to_value(self) -> Value {
        Value::Integer(self as i64)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_integer().map(|e| e as i32)
    }
}

impl RegistryType for bool {
    fn to_value(self) -> Value {
        Value::Boolean(self)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_bool()
    }
}

impl RegistryType for f32 {
    fn to_value(self) -> Value {
        Value::Float(self as f64)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_float().map(|e| e as f32)
    }
}

impl RegistryType for f64 {
    fn to_value(self) -> Value {
        Value::Float(self)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_float()
    }
}

impl RegistryType for String {
    fn to_value(self) -> Value {
        Value::String(self)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_str().map(|v| v.to_string())
    }
}

impl<T> RegistryType for [T; 2]
    where T: RegistryType + Clone
{
    fn to_value(self) -> Value {
        Value::Array(self.iter().map(|e| T::to_value(e.clone())).collect())
    }

    fn from_value(v: &Value) -> Option<Self> {
        match v.as_slice() {
            Some(s) => {
                if s.len() >= 2 {
                    Some([match T::from_value(&s[0]) {
                              Some(x) => x,
                              None => return None,
                          },
                          match T::from_value(&s[1]) {
                              Some(x) => x,
                              None => return None,
                          }])
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
