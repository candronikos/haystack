use mlua::prelude::*;
use mlua::{Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use crate::lua::lany::HAny;
use crate::{io, h_list::HList, HVal, NumTrait};
use crate::lua::{H,LuaFloat};

impl <'a: 'static>UserData for H<HList<'a, LuaFloat>> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      let mut out = String::new();
      this.to_zinc(&mut out)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
      Ok(out)
    });

    methods.add_meta_method(MetaMethod::Index, |_, this, (idx,): (i64,)| {
      let tmp_idx: usize = if idx.is_positive() { idx as usize - 1 } else { this.len() - (idx*-1) as usize };

      let res = match this.get(tmp_idx) {
        Some(element) => Some(HAny::from_hval(element.clone())),
        None => None,
      };

      Ok(res)
    });

    methods.add_meta_method(MetaMethod::Len, |_, this, ()| {
      Ok(this.len())
    });
  }
}