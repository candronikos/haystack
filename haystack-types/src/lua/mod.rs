use std::ops::Deref;
use std::rc::Rc;

use mlua::prelude::*;
use mlua::{Lua, UserData, MetaMethod, Error as LuaError, Result as LuaResult, Table as LuaTable};
use nom::Parser;
use crate::h_dict::HDict;
use crate::{io, HGrid, HRow, HVal, NumTrait};

pub type LuaFloat = f64;

pub struct HWrapper<T> {
    pub inner: Rc<T>
}

impl<T> HWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { inner: Rc::new(inner) as Rc<T> }
    }

    fn get_ref(&self) -> Rc<T> {
        self.inner.clone()
    }
}

impl<'a, T> Deref for HWrapper<T> {
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
        Ok(HWrapper::new(grid))
      })?
  )?;

  io_table.set("grid",io_grid_table)?;
  hs_table.set("io",io_table,)?;
  Ok(hs_table)
}

impl <'a: 'static>UserData for HWrapper<HGrid<'a, LuaFloat>> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
      methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
        let mut out = String::new();
        this.to_zinc(&mut out)
          .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        Ok(out)
      });

      methods.add_meta_method(MetaMethod::Index, |_, this, (idx,): (i64,)| {
        let tmp_idx: usize = if idx.is_positive() { idx as usize - 1 } else { this.len() - (idx*-1) as usize };

        match this.get(tmp_idx) {
          Ok(row) => Ok(HWrapper::new(row.to_dict())),
          Err(_) => Err(LuaError::RuntimeError(format!("Row index {} out of bounds", idx))),
        }
      });
    }
}

impl<'a, T: NumTrait> UserData for HWrapper<HDict<'a, T>> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      let mut out = String::new();
      this.to_zinc(&mut out)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
      Ok(out)
    });

    /*
    methods.add_meta_method(MetaMethod::Index, |_, this, (key,): (String,)| {
      this.get(&key)
        .cloned()
        .ok_or_else(|| LuaError::RuntimeError(format!("Key '{}' not found", key)))
    });

    methods.add_method("keys", |_, this, ()| {
      let keys: Vec<String> = this.keys().cloned().collect();
      Ok(keys)
    });

    methods.add_method("values", |_, this, ()| {
      let values: Vec<HVal> = this.values().cloned().collect();
      Ok(values)
    });
    */
  }
}