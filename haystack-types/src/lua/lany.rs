use mlua::prelude::*;
use mlua::{UserData, MetaMethod, Error as LuaError};
use crate::h_bool::HBool;
use crate::h_val::HBox;
use crate::{h_marker, h_null, h_remove, h_na, HType};
use crate::{h_null::HNull, h_marker::HMarker, h_remove::HRemove, h_na::HNA, Dict, List,  HGrid, HRow, HVal, NumTrait, Number, Ref, Str, XStr, Symbol, Uri, Coord, Date, Time, DateTime};
use crate::lua::{H,LuaFloat};

#[derive(Clone)]
pub enum HAny<'a> {
  Null,
  Marker,
  Remove,
  NA,
  Bool(HBool),
  Number(Number<LuaFloat>),
  Str(Str),
  Uri(Uri),
  Ref(Ref),
  Symbol(Symbol),
  Date(Date),
  Time(Time),
  DateTime(DateTime),
  Coord(Coord<LuaFloat>),
  XStr(XStr),
  List(List<'a,LuaFloat>),
  Dict(Dict<'a,LuaFloat>),
  Grid(HGrid<'a,LuaFloat>),
}

impl <'a>HAny<'a> {
  pub fn from_hval(val: HBox<'a,LuaFloat>) ->  HAny<'a> {
    let val_ref = val.as_ref();
    match val.haystack_type() {
      HType::Null => HAny::Null,
      HType::Marker => HAny::Marker,
      HType::Remove => HAny::Remove,
      HType::NA => HAny::NA,
      HType::Bool => val.get_bool_val().map(|v| HAny::Bool(v.clone())).unwrap(),
      HType::Number => val.get_number_val().map(|v| HAny::Number(v.clone())).unwrap(),
      HType::Str => val.get_string_val().map(|v| HAny::Str(v.clone())).unwrap(),
      HType::Uri => val.get_uri_val().map(|v| HAny::Uri(v.clone())).unwrap(),
      HType::Ref => val.get_ref_val().map(|v| HAny::Ref(v.clone())).unwrap(),
      HType::Symbol => val.get_symbol_val().map(|v| HAny::Symbol(v.clone())).unwrap(),
      HType::Date => val.get_date_val().map(|v| HAny::Date(v.clone())).unwrap(),
      HType::Time => val.get_time_val().map(|v| HAny::Time(v.clone())).unwrap(),
      HType::DateTime => val.get_datetime_val().map(|v| HAny::DateTime(v.clone())).unwrap(),
      HType::Coord => val.get_coord_val().map(|v| HAny::Coord(v.clone())).unwrap(),
      HType::XStr => val.get_xstr_val().map(|v| HAny::XStr(v.clone())).unwrap(),
      HType::List => val_ref.get_list_val().map(|v| HAny::List(v.clone())).unwrap(),
      HType::Dict => val_ref.get_dict_val().map(|v| HAny::Dict(v.clone())).unwrap(),
      HType::Grid => val_ref.get_grid_val().map(|v| HAny::Grid(v.clone())).unwrap(),
      _ => panic!("Unsupported Haystack type: {:?}", val.haystack_type()),
    }
  }
  pub fn to_hval(&self) -> &dyn HVal<'a, LuaFloat> {
    match self {
      HAny::Null => &h_null::NULL,
      HAny::Marker => &h_marker::MARKER,
      HAny::Remove => &h_remove::REMOVE,
      HAny::NA => &h_na::NA,
      HAny::Bool(b) => b,
      HAny::Number(n) => n,
      HAny::Str(s) => s,
      HAny::Uri(u) => u,
      HAny::Ref(r) => r,
      HAny::Symbol(s) => s,
      HAny::Date(d) => d,
      HAny::Time(t) => t,
      HAny::DateTime(dt) => dt,
      HAny::Coord(c) => c,
      HAny::XStr(x) => x,
      HAny::List(l) => l,
      HAny::Dict(d) => d,
      HAny::Grid(g) => g,
    }
  }
  pub fn to_lua_null(&self) -> Option<H<HNull>> {
    match self {
      HAny::Null => Some(H::new(HNull)),
      _ => None,
    }
  }
  pub fn to_lua_marker(&self) -> Option<H<HMarker>> {
    match self {
      HAny::Marker => Some(H::new(HMarker)),
      _ => None,
    }
  }
  pub fn to_lua_remove(&self) -> Option<H<HRemove>> {
    match self {
      HAny::Remove => Some(H::new(HRemove)),
      _ => None,
    }
  }
  pub fn to_lua_na(&self) -> Option<H<HNA>> {
    match self {
      HAny::NA => Some(H::new(HNA)),
      _ => None,
    }
  }
  pub fn to_lua_bool(&self) -> Option<H<HBool>> {
    match self {
      HAny::Bool(b) => Some(H::new(b.clone())),
      _ => None,
    }
  }
  pub fn to_lua_number(&self) -> Option<H<Number<LuaFloat>>> {
    match self {
      HAny::Number(n) => Some(H::new(n.clone())),
      _ => None,
    }
  }
  pub fn to_lua_str(&self) -> Option<H<Str>> {
    match self {
      HAny::Str(s) => Some(H::new(s.clone())),
      _ => None,
    }
  }
  pub fn to_lua_uri(&self) -> Option<H<Uri>> {
    match self {
      HAny::Uri(u) => Some(H::new(u.clone())),
      _ => None,
    }
  }
  pub fn to_lua_ref(&self) -> Option<H<Ref>> {
    match self {
      HAny::Ref(r) => Some(H::new(r.clone())),
      _ => None,
    }
  }
  pub fn to_lua_symbol(&self) -> Option<H<Symbol>> {
    match self {
      HAny::Symbol(s) => Some(H::new(s.clone())),
      _ => None,
    }
  }
  pub fn to_lua_date(&self) -> Option<H<Date>> {
    match self {
      HAny::Date(d) => Some(H::new(d.clone())),
      _ => None,
    }
  }
  pub fn to_lua_time(&self) -> Option<H<Time>> {
    match self {
      HAny::Time(t) => Some(H::new(t.clone())),
      _ => None,
    }
  }
  pub fn to_lua_datetime(&self) -> Option<H<DateTime>> {
    match self {
      HAny::DateTime(dt) => Some(H::new(dt.clone())),
      _ => None,
    }
  }
  pub fn to_lua_coord(&self) -> Option<H<Coord<LuaFloat>>> {
    match self {
      HAny::Coord(c) => Some(H::new(c.clone())),
      _ => None,
    }
  }
  pub fn to_lua_xstr(&self) -> Option<H<XStr>> {
    match self {
      HAny::XStr(x) => Some(H::new(x.clone())),
      _ => None,
    }
  }
  pub fn to_lua_list(&self) -> Option<H<List<'a, LuaFloat>>> {
    match self {
      HAny::List(l) => Some(H::new(l.clone())),
      _ => None,
    }
  }
  pub fn to_lua_dict(&self) -> Option<H<Dict<'a, LuaFloat>>> {
    match self {
      HAny::Dict(d) => Some(H::new(d.clone())),
      _ => None,
    }
  }
  pub fn to_lua_grid(&self) -> Option<H<HGrid<'a, LuaFloat>>> {
    match self {
      HAny::Grid(g) => Some(H::new(g.clone())),
      _ => None,
    }
  }
}

impl<'a: 'static> UserData for HAny<'a> {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
      let mut out = String::new();
      this.to_hval().to_zinc(&mut out)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
      Ok(out)
    });
  }
}