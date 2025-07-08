use mlua::prelude::*;
use mlua::{Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Value};

use crate::h_bool::HBool;
use crate::lua::{H};

impl LuaUserData for H<HBool> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      if this.0 {
        Ok("true")
      } else {
        Ok("false")
      }
    });

    methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: Value| {
      match other {
        LuaValue::UserData(ud) => {
          if let Ok(_) = ud.borrow::<HBool>() {
            Ok(true)
          } else {
            Ok(false)
          }
        }
        _ => Ok(false)
      }
    });
  }
}