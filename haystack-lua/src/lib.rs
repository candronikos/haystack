use std::ops::Deref;
use std::rc::Rc;

#[cfg(feature = "lua51")]
extern crate mlua_51 as mlua;
#[cfg(feature = "lua52")]
extern crate mlua_52 as mlua;
#[cfg(feature = "lua53")]
extern crate mlua_53 as mlua;
#[cfg(feature = "lua54")]
extern crate mlua_54 as mlua;
#[cfg(feature = "luajit52")]
extern crate mlua_jit52 as mlua;
#[cfg(feature = "luajit")]
extern crate mlua_luajit as mlua;
#[cfg(feature = "luau")]
extern crate mlua_u as mlua;
#[cfg(feature = "luau-jit")]
extern crate mlua_ujit as mlua;
#[cfg(feature = "luau-vector4")]
extern crate mlua_uvector4 as mlua;

use haystack_types::h_number::HNumber;
use haystack_types::h_val::HBox;
use haystack_types::{Parser, HGrid, HRow, HType, HVal, NumTrait, io};
use mlua::prelude::*;
use mlua::{
    Function as LuaFunction, Error as LuaError, Lua, MetaMethod, Result as LuaResult, Table as LuaTable, UserData, Value,
};

mod lbool;
mod lcol;
mod lcoord;
mod ldate;
mod ldatetime;
mod ldict;
mod lgrid;
mod llist;
mod lmarker;
mod lna;
mod lnull;
mod lnumber;
mod lref;
mod lremove;
mod lstr;
mod lsymbol;
mod ltime;
mod luri;
mod lxstr;

pub type LuaFloat = f64;

pub struct H<T> {
    pub inner: Rc<T>,
}

impl<T> H<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(inner) as Rc<T>,
        }
    }

    fn get_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<'a, T> Deref for H<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

#[mlua::lua_module]
pub fn haystack(lua: &Lua) -> LuaResult<LuaTable> {
    let hs_table = lua.create_table()?;
    let io_table = lua.create_table()?;
    let parse_table = lua.create_table()?;
    let zinc_table = lua.create_table()?;

    setup_tonumber_override(lua)?;
    zinc_table.set(
        "grid",
        lua.create_function(|_, args: String| {
            let (_, grid) = io::parse::zinc::grid::<LuaFloat>
                .parse(args.as_str())
                .or_else(|e| Err(LuaError::RuntimeError(e.to_string())))?;
            Ok(H::new(grid))
        })?,
    )?;

    zinc_table.set(
        "list",
        lua.create_function(|_, args: String| {
            let (_, grid) = io::parse::zinc::list::<LuaFloat>
                .parse(args.as_str())
                .or_else(|e| Err(LuaError::RuntimeError(e.to_string())))?;
            Ok(H::new(grid))
        })?,
    )?;

    zinc_table.set(
        "dict",
        lua.create_function(|_, args: String| {
            let (_, grid) = io::parse::zinc::dict::<LuaFloat>
                .parse(args.as_str())
                .or_else(|e| Err(LuaError::RuntimeError(e.to_string())))?;
            Ok(H::new(grid))
        })?,
    )?;

    parse_table.set("zinc", zinc_table)?;
    io_table.set("parse", parse_table)?;
    hs_table.set("io", io_table)?;
    Ok(hs_table)
}

pub fn create_lua_data(lua: &Lua, value: HBox<'static, LuaFloat>) -> LuaResult<Value> {
    let l_type = match value.haystack_type() {
        HType::Null => lua.create_userdata(H::new(haystack_types::h_null::HNull))?,
        HType::Marker => lua.create_userdata(H::new(haystack_types::h_marker::HMarker))?,
        HType::Remove => lua.create_userdata(H::new(haystack_types::h_remove::HRemove))?,
        HType::NA => lua.create_userdata(H::new(haystack_types::h_na::HNA))?,
        HType::Bool => lua.create_userdata(H::new(value.get_bool_val().unwrap().clone()))?,
        HType::Number => lua.create_userdata(H::new(value.get_number_val().unwrap().clone()))?,
        HType::Str => lua.create_userdata(H::new(value.get_string_val().unwrap().clone()))?,
        HType::Uri => lua.create_userdata(H::new(value.get_uri_val().unwrap().clone()))?,
        HType::Ref => lua.create_userdata(H::new(value.get_ref_val().unwrap().clone()))?,
        HType::Symbol => lua.create_userdata(H::new(value.get_symbol_val().unwrap().clone()))?,
        HType::Date => lua.create_userdata(H::new(value.get_date_val().unwrap().clone()))?,
        HType::Time => lua.create_userdata(H::new(value.get_time_val().unwrap().clone()))?,
        HType::DateTime => {
            lua.create_userdata(H::new(value.get_datetime_val().unwrap().clone()))?
        }
        HType::Coord => lua.create_userdata(H::new(value.get_coord_val().unwrap().clone()))?,
        HType::XStr => lua.create_userdata(H::new(value.get_xstr_val().unwrap().clone()))?,
        HType::List => lua.create_userdata(H::new(value.get_list_val().unwrap().clone()))?,
        HType::Dict => lua.create_userdata(H::new(value.get_dict_val().unwrap().clone()))?,
        HType::Grid => lua.create_userdata(H::new(value.get_grid_val().unwrap().clone()))?,
    };

    Ok(Value::UserData(l_type))
}

pub fn setup_tonumber_override(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let original_tonumber: LuaFunction = globals.get("tonumber")?;

    let enhanced_tonumber = lua.create_function(move |lua, value: Value| match value {
        Value::UserData(ud) => {
            if let Ok(hnumber) = ud.borrow::<H<HNumber<LuaFloat>>>() {
                Ok(Value::Number(hnumber.val()))
            } else {
                Ok(Value::Nil)
            }
        }
        _ => LuaFunction::call::<Value>(&original_tonumber, value),
    })?;

    globals.set("tonumber", enhanced_tonumber)?;

    Ok(())
}
