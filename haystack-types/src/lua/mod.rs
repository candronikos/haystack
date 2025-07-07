use std::ops::Deref;
use std::rc::Rc;

use mlua::prelude::*;
use mlua::{Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use nom::Parser;
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
  let io_grid_table = lua.create_table()?;
  let io_table = lua.create_table()?;

  io_grid_table.set(
      "from_zinc",
      lua.create_function(|_, args: String| {
        let (_, grid) = io::parse::zinc::grid::<LuaFloat>
          .parse(args.as_str())
          .or_else(|e| Err(LuaError::RuntimeError(e.to_string())))?;
        Ok(H::new(grid))
      })?
  )?;

  io_table.set("grid",io_grid_table)?;
  hs_table.set("io",io_table,)?;
  Ok(hs_table)
}