use mlua::prelude::*;
use mlua::{Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use nom::{Parser,combinator::all_consuming};
use crate::h_grid::HGridErr;
use crate::NumTrait;
use crate::{io, HGrid, HVal};

#[mlua::lua_module]
pub fn haystack(lua: &Lua) -> LuaResult<LuaTable> {
  let hs_table = lua.create_table()?;
  let io_grid_table = lua.create_table()?;
  let io_table = lua.create_table()?;

  io_grid_table.set(
      "from_zinc",
      lua.create_function(|_, args: String| {
        let (_, grid) = io::parse::zinc::grid::<f64> //all_consuming(io::parse::zinc::grid::<f64>)
          .parse(args.as_str())
          .or_else(|e| Err(LuaError::RuntimeError(e.to_string())))?;
        Ok(grid)
      })?
  )?;

  io_table.set("grid",io_grid_table)?;
  hs_table.set("io",io_table,)?;
  Ok(hs_table)
}

impl <'a,T: NumTrait>UserData for HGrid<'a, T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
      methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
        let mut out = String::new();
        this.to_zinc(&mut out)
          .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        Ok(out)
      });
    }
}