use crate::lua::{H, LuaFloat};
use crate::{HVal, NumTrait, h_symbol::HSymbol, io};
use mlua::prelude::*;
use mlua::{
    AnyUserData, Error as LuaError, Lua, MetaMethod, Result as LuaResult, Table as LuaTable,
    UserData,
};

impl<'a: 'static> UserData for H<HSymbol> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut buf = String::new();
            HVal::<LuaFloat>::to_zinc(this.get_ref(), &mut buf).unwrap();
            Ok(buf)
        });
    }
}
