use crate::{H, LuaFloat, create_lua_data};
use haystack_types::HCol;
use mlua::prelude::*;
use mlua::{Error as LuaError, MetaMethod, UserData};
use std::fmt::Write;

impl<'a: 'static> UserData for H<HCol<'a, LuaFloat>> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.to_owned()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut out = String::new();
            write!(out, "HCol<{}>", this.name)
                .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            Ok(out)
        });

        methods.add_meta_method(MetaMethod::Index, |lua, this, (key,): (String,)| match this
            .get(key)
            .map(|v| create_lua_data(lua, v.clone()))
        {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(e)) => Err(LuaError::RuntimeError(e.to_string())),
            None => Ok(None),
        });

        methods.add_method("meta", |_, this, ()| {
            let meta = this.meta().clone();
            let ret = H::new(meta);
            Ok(ret)
        });

        methods.add_method("has", |_, this, (col_name,): (String,)| {
            Ok(this.has(&col_name))
        });
    }
}
