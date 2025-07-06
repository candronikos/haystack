mod err;
pub use err::HError;

mod common;
pub use common::{Txt,ZincReader,ZincWriter};

#[macro_use]
mod h_val;
pub use h_val::{HVal,HType};

mod cast;
pub use cast::*;

mod h_null;
pub use h_null::NULL;

mod h_marker;
pub use h_marker::MARKER;

mod h_remove;
pub use h_remove::REMOVE;

mod h_bool;
pub use h_bool::Bool;

mod h_na;
pub use h_na::NA;

mod h_number;
pub use h_number::{Number, NumTrait, HUnit as Unit};
pub use num::Float;

mod h_str;
pub use h_str::Str;

mod h_xstr;
pub use h_xstr::XStr;

mod h_uri;
pub use h_uri::Uri;

mod h_ref;
pub use h_ref::Ref;

mod h_symbol;
pub use h_symbol::Symbol;

mod h_date;
pub use h_date::Date;

mod h_time;
pub use h_time::Time;

mod h_datetime;
pub use h_datetime::DateTime;

mod h_coord;
pub use h_coord::Coord;

mod h_grid;
pub use h_grid::{HGrid,HCol,HRow};

mod h_dict;
pub use h_dict::Dict;

mod h_list;
pub use h_list::List;

pub mod io;

#[cfg(feature = "lua54")]
extern crate mlua_54 as mlua;
#[cfg(feature = "lua53")]
extern crate mlua_53 as mlua;
#[cfg(feature = "lua52")]
extern crate mlua_52 as mlua;
#[cfg(feature = "lua51")]
extern crate mlua_51 as mlua;
#[cfg(feature = "luajit")]
extern crate mlua_luajit as mlua;
#[cfg(feature = "luajit52")]
extern crate mlua_jit52 as mlua;
#[cfg(feature = "luau")]
extern crate mlua_u as mlua;
#[cfg(feature = "luau-jit")]
extern crate mlua_ujit as mlua;
#[cfg(feature = "luau-vector4")]
extern crate mlua_uvector4 as mlua;

#[cfg(feature = "lua")]
pub mod lua;
#[cfg(feature = "lua")]
pub use lua::{haystack, H, LuaFloat};