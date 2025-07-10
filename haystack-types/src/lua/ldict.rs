use crate::lua::{H, LuaFloat, create_lua_data};
use crate::{Dict, HRow, HVal, NumTrait, h_val::HType};
use mlua::prelude::*;
use mlua::{
    Error as LuaError, Lua, MetaMethod, Result as LuaResult, Table as LuaTable, UserData, Value,
};

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
                Some(value) => Some(create_lua_data(lua, value.clone())?),
                None => None,
            };

            Ok(res)
        });
    }
}
