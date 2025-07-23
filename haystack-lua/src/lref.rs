use crate::{H, LuaFloat};
use haystack_types::io::write::ZincWriter;
use haystack_types::{HVal, NumTrait, h_ref::HRef, io};
use mlua::prelude::*;
use mlua::{
    AnyUserData, Error as LuaError, Lua, MetaMethod, Result as LuaResult, Table as LuaTable,
    UserData,
};
use std::fmt::Write;

impl<'a: 'static> UserData for H<HRef> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id.to_owned()));
        fields.add_field_method_get("dis", |_, this| Ok(this.dis.to_owned()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut buf = String::new();
            write!(buf, "{}", ZincWriter::new(this.get_ref())).unwrap();
            Ok(buf)
        });
    }
}
