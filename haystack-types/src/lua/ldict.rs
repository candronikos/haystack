use mlua::prelude::*;
use mlua::{Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use crate::lua::lany::HAny;
use crate::{Dict, HRow, HVal, NumTrait};
use crate::lua::{H,LuaFloat};

impl<'a: 'static> UserData for H<Dict<'a, LuaFloat>> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      let mut out = String::new();
      this.to_zinc(&mut out)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
      Ok(out)
    });

    methods.add_meta_method(MetaMethod::Index, |_, this, (key,): (String,)| {
      let res = match this.get(&key) {
        Some(value) => Some(HAny::from_hval(value.clone())),
        None => None,
      };

      Ok(res)
    });

    /*
    methods.add_method("keys", |_, this, ()| {
      let keys: Vec<String> = this.keys().cloned().collect();
      Ok(keys)
    });

    methods.add_method("values", |_, this, ()| {
      let values: Vec<HVal> = this.values().cloned().collect();
      Ok(values)
    });
    */
  }
}