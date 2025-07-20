use crate::{H, LuaFloat};
use haystack_types::io::write::ZincWriter;
use haystack_types::{HVal, h_coord::HCoord, io};
use mlua::prelude::*;
use mlua::{
    AnyUserData, Error as LuaError, Lua, MetaMethod, Result as LuaResult, Table as LuaTable,
    UserData,
};

impl<'a: 'static> UserData for H<HCoord<LuaFloat>> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut buf = String::new();
            ZincWriter::to_zinc(this.get_ref(), &mut buf).unwrap();
            Ok(buf)
        });
    }
}
