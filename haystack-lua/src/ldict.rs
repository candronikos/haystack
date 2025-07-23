use crate::{H, LuaFloat, create_lua_data};
use haystack_types::io::write::ZincWriter;
use haystack_types::{Dict, HRow, HVal, NumTrait, h_val::HType};
use haystack_types::{Float, HGrid};
use mlua::prelude::*;
use mlua::{
    Error as LuaError, Lua, MetaMethod, Result as LuaResult, Table as LuaTable, UserData, Value,
};
use std::fmt::Write;

impl<'a: 'static> UserData for H<Dict<'a, LuaFloat>> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut out = String::new();
            write!(out, "{}", ZincWriter::new(this.get_ref())).unwrap();
            Ok(out)
        });

        methods.add_meta_method(MetaMethod::Index, |lua, this, (key,): (String,)| {
            let res = match this.get(&key) {
                Some(value) => Some(create_lua_data(lua, value.clone())?),
                None => None,
            };

            Ok(res)
        });

        methods.add_method("has", |_, this, (key,): (String,)| Ok(this.has(&key)));
    }
}

pub fn to_dict(row: HRow<LuaFloat>) -> Dict<LuaFloat> {
    let mut dict = Dict::new();
    for (idx, col) in row.cols.iter().enumerate() {
        let inner = &row.inner;
        if let Some(val) = inner.upgrade().unwrap().get(idx) {
            if let Some(v) = val {
                dict.set(col.name.to_owned(), v.clone());
            }
        }
    }
    dict
}
