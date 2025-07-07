use std::ops::Deref;
use std::rc::Rc;

use chrono::format::parse;
use mlua::prelude::*;
use mlua::{Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use nom::Parser;
use crate::io::parse::zinc::dict;
use crate::{io, HGrid, HRow, HVal, NumTrait};

mod lany;
mod lgrid;
mod llist;
mod ldict;

pub type LuaFloat = f64;

pub struct H<T> {
    pub inner: Rc<T>
}

impl<T> H<T> {
    pub fn new(inner: T) -> Self {
        Self { inner: Rc::new(inner) as Rc<T> }
    }

    fn get_ref(&self) -> Rc<T> {
        self.inner.clone()
    }
}

impl<'a, T> Deref for H<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[mlua::lua_module]
pub fn haystack(lua: &Lua) -> LuaResult<LuaTable> {
  let hs_table = lua.create_table()?;
  let io_table = lua.create_table()?;
  let parse_table = lua.create_table()?;
  let grid_table = lua.create_table()?;
  let list_table = lua.create_table()?;
  let dict_table = lua.create_table()?;

    grid_table.set(
        "zinc",
        lua.create_function(|_, args: String| {
        let (_, grid) = io::parse::zinc::grid::<LuaFloat>
            .parse(args.as_str())
            .or_else(|e| Err(LuaError::RuntimeError(e.to_string())))?;
        Ok(H::new(grid))
        })?
    )?;

    list_table.set(
        "zinc",
        lua.create_function(|_, args: String| {
        let (_, grid) = io::parse::zinc::list::<LuaFloat>
            .parse(args.as_str())
            .or_else(|e| Err(LuaError::RuntimeError(e.to_string())))?;
        Ok(H::new(grid))
        })?
    )?;

    dict_table.set(
        "zinc",
        lua.create_function(|_, args: String| {
        let (_, grid) = io::parse::zinc::dict::<LuaFloat>
            .parse(args.as_str())
            .or_else(|e| Err(LuaError::RuntimeError(e.to_string())))?;
        Ok(H::new(grid))
        })?
    )?;

    parse_table.set("list",list_table)?;
    parse_table.set("dict",dict_table)?;
    parse_table.set("grid",grid_table)?;
    io_table.set("parse",parse_table)?;
    hs_table.set("io",io_table,)?;
    Ok(hs_table)
}