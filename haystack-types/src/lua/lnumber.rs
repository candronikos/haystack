use mlua::prelude::*;

use crate::h_number::HNumber;
use crate::lua::{H,LuaFloat};

impl LuaUserData for H<HNumber<LuaFloat>> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(LuaMetaMethod::ToString, |_, _this, ()| {
      Ok("M")
    });

    methods.add_meta_method(LuaMetaMethod::Eq, |_, _this, other: LuaValue| {
      match other {
        LuaValue::UserData(ud) => {
          if let Ok(_) = ud.borrow::<Self>() {
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