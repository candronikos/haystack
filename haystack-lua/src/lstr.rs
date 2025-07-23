use crate::H;
use haystack_types::h_str::HStr;
use mlua::prelude::*;
use mlua::{Error as LuaError, Lua, MetaMethod, Result as LuaResult, UserData, Value};

impl<'a: 'static> UserData for H<HStr> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.clone_into_string())
        });

        methods.add_meta_method(MetaMethod::Eq, |_, this, other: Value| match other {
            mlua::Value::String(lua_str) => match lua_str.to_str() {
                Ok(other_str) => Ok(this.as_str() == &*other_str),
                Err(_) => Ok(false),
            },
            mlua::Value::UserData(user_data) => {
                if let Ok(other_hstr) = user_data.borrow::<H<HStr>>() {
                    Ok(this.as_str() == other_hstr.as_str())
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        });

        methods.add_meta_method(MetaMethod::Len, |_, this, ()| Ok(this.len()));

        methods.add_method("is_empty", |_, this, ()| Ok(this.is_empty()));
    }
}

impl FromLua for H<HStr> {
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        match value {
            Value::String(lua_str) => {
                let str_value = lua_str.to_str()?;
                Ok(H::new(HStr::new((&*str_value).to_owned())))
            }
            Value::UserData(user_data) => {
                let hstr = user_data.borrow::<H<HStr>>()?;
                Ok(H::new(hstr.get_ref().clone()))
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "HStr".to_owned(),
                message: Some("expected string or HStr".to_string()),
            }),
        }
    }
}
