use mlua::prelude::*;
use mlua::{AnyUserData, Lua, Value, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use crate::{io, h_list::HList, HVal, NumTrait, HType};
use crate::lua::{create_lua_data, LuaFloat, H};

impl <'a: 'static>UserData for H<HList<'a, LuaFloat>> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      let mut out = String::new();
      this.to_zinc(&mut out)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
      Ok(out)
    });

    methods.add_meta_method(MetaMethod::Index, |lua, this, (idx,): (i64,)| {
      let tmp_idx: usize = if idx.is_positive() { idx as usize - 1 } else { this.len() - (idx*-1) as usize };
      
      let res = match this.get(tmp_idx) {
        Some(value) => {
          let l_type = create_lua_data(lua, value.clone())?;
          Some(l_type)
        },
        None => None,
      };

      Ok(res)
    });

    methods.add_meta_method(MetaMethod::Len, |_, this, ()| {
      Ok(this.len())
    });

    methods.add_method("is_empty", |_, this, ()| {
      Ok(this.is_empty())
    });

    methods.add_method("first", |lua, this, ()| {
      match this.get_ref().first() {
        Some(element) => {
          let l_type = create_lua_data(lua, element.clone())?;
          Ok(Some(l_type))
        },
        None => Ok(None),
      }
    });

    methods.add_method("last", |_, this, ()| {
      match this.last() {
        Some(element) => Ok(Some(AnyUserData::wrap(H::new(element.to_owned())))),
        None => Ok(None),
      }
    });
  }
}