use mlua::prelude::*;
use mlua::{AnyUserData, Value, Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use crate::{io, h_str::HStr, HVal, NumTrait};
use crate::lua::{H,LuaFloat};

impl <'a: 'static>UserData for H<HStr> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      Ok(this.clone_into_string())
    });

    methods.add_meta_method(MetaMethod::Eq, |_, this, other: Value| {
      match other {
        Value::UserData(ud) => {
          if let Ok(rhs) = ud.borrow::<HStr>() {
            Ok(this.0 == rhs.0)
          } else {
            Ok(false)
          }
        }
        Value::String(s) => {
          Ok(s.as_bytes() == this.0.as_bytes())
        }
        _ => Ok(false)
      }
    });

    methods.add_meta_method(MetaMethod::Len, |_, this, ()| {
      Ok(this.len())
    });

    methods.add_method("is_empty", |_, this, ()| {
      Ok(this.is_empty())
    });
  }
}