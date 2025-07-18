use std::rc::Rc;

use crate::ldict::to_dict;
use crate::{H, LuaFloat};
use crate::{HGrid, HVal};
use haystack_types::h_list::HList;
use haystack_types::h_val::HBox;
use mlua::prelude::*;
use mlua::{Error as LuaError, Lua, MetaMethod, Result as LuaResult, Table as LuaTable, UserData};

impl<'a: 'static> UserData for H<HGrid<'a, LuaFloat>> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut out = String::new();
            this.to_zinc(&mut out)
                .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            Ok(out)
        });

        methods.add_meta_method(MetaMethod::Index, |_, this, (idx,): (i64,)| {
            let tmp_idx: usize = if idx.is_positive() {
                idx as usize - 1
            } else {
                this.len() - (idx * -1) as usize
            };

            match this.get(tmp_idx) {
                Ok(row) => Ok(Some(H::new(to_dict(row)))),
                Err(_) => Ok(None),
            }
        });

        methods.add_meta_method(MetaMethod::Len, |_, this, ()| Ok(this.len()));

        methods.add_method("meta", |_, this, ()| {
            let meta = this.meta().clone();
            let ret = H::new(meta);
            Ok(ret)
        });

        methods.add_method("cols", |_, this, ()| {
            let cols = this
                .iter_cols()
                .map(|c| H::new(c.clone()))
                .collect::<Vec<_>>();
            Ok(cols)
        });

        methods.add_method("is_empty", |_, this, ()| Ok(this.is_empty()));

        methods.add_method("first", |_, this, ()| match this.first() {
            Ok(row) => Ok(Some(H::new(to_dict(row)))),
            Err(_) => Ok(None),
        });

        methods.add_method("last", |_, this, ()| match this.last() {
            Ok(row) => Ok(Some(H::new(to_dict(row)))),
            Err(_) => Ok(None),
        });

        methods.add_method("has", |_, this, (col_name,): (String,)| {
            Ok(this.has(&col_name))
        });

        /*
        methods.add_method("add_meta", |_, this, (meta,): (H<HDict<LuaFloat>>,)| {
          let inner = match this.as_ref().add_meta(meta.into_map()) {
            Ok(grid) => grid,
            Err(e) => return Err(LuaError::RuntimeError(e.to_string())),
          };
          let ret = H::new(inner);
          Ok(ret)
        });
        */
    }
}
