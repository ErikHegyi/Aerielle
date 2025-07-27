use std::collections::HashMap;

pub type Context = HashMap<String, String>;

/*
#[macro_export]
macro_rules! context {
    { $($key: tt : $value: tt),* } => {
        {
            use crate::html::Context;
            let mut map: Context = Context::new();
            $(
                map.insert(
                    $key.to_string(),
                    $value.to_string()
                );
            )*
            map
        }
    }
}*/