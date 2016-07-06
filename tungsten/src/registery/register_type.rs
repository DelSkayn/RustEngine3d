
use toml::Value;

pub trait RegisteryType: Sized {
    fn to_value(self) -> Value;
    fn from_value(v: &Value) -> Option<Self>;
}


impl RegisteryType for u64 {
    fn to_value(self) -> Value {
        Value::Integer(self as i64)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_integer().map(|e| e as u64)
    }
}

impl RegisteryType for i64 {
    fn to_value(self) -> Value {
        Value::Integer(self)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_integer()
    }
}

impl RegisteryType for u32 {
    fn to_value(self) -> Value {
        Value::Integer(self as i64)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_integer().map(|e| e as u32)
    }
}

impl RegisteryType for i32 {
    fn to_value(self) -> Value {
        Value::Integer(self as i64)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_integer().map(|e| e as i32)
    }
}

impl RegisteryType for bool {
    fn to_value(self) -> Value {
        Value::Boolean(self)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_bool()
    }
}

impl RegisteryType for f32 {
    fn to_value(self) -> Value {
        Value::Float(self as f64)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_float().map(|e| e as f32)
    }
}

impl RegisteryType for f64 {
    fn to_value(self) -> Value {
        Value::Float(self)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_float()
    }
}

impl RegisteryType for String {
    fn to_value(self) -> Value {
        Value::String(self)
    }

    fn from_value(v: &Value) -> Option<Self> {
        v.as_str().map(|v| v.to_string())
    }
}

impl<T> RegisteryType for [T; 2]
    where T: RegisteryType + Clone
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
