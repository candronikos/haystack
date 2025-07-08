use mlua::prelude::*;
use mlua::{Lua, Value, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use crate::{Dict, HRow, HVal, NumTrait, h_val::HType};
use crate::lua::{create_lua_data, LuaFloat, H};

impl<'a: 'static> UserData for H<Dict<'a, LuaFloat>> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      let mut out = String::new();
      this.to_zinc(&mut out)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
      Ok(out)
    });

    methods.add_meta_method(MetaMethod::Index, |lua, this, (key,): (String,)| {
      let res = match this.get(&key) {
        Some(value) => {
          Some(create_lua_data(lua, value.clone())?)
        },
        None => None,
      };

      Ok(res)
    });
  }
}