use mlua::prelude::*;
use mlua::{AnyUserData, Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use crate::{io, h_datetime::HDateTime, HVal, NumTrait};
use crate::lua::{H,LuaFloat};

impl <'a: 'static>UserData for H<HDateTime> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      let mut buf = String::new();
      HVal::<LuaFloat>::to_zinc(this.get_ref(), &mut buf).unwrap();
      Ok(buf)
    });
  }
}