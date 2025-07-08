use mlua::prelude::*;
use mlua::{AnyUserData, Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use crate::{io, h_uri::HUri, HVal, NumTrait};
use crate::lua::{H,LuaFloat};

impl <'a: 'static>UserData for H<HUri> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      Ok(this.to_owned_string())
    });
  }
}