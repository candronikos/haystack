use crate::lua::{H, LuaFloat};
use crate::{HVal, NumTrait, h_xstr::HXStr, io};
use mlua::prelude::*;
use mlua::{
    AnyUserData, Error as LuaError, Lua, MetaMethod, Result as LuaResult, Table as LuaTable,
    UserData,
};

impl<'a: 'static> UserData for H<HXStr> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut buf = String::new();
            HVal::<LuaFloat>::to_zinc(this.get_ref(), &mut buf).unwrap();
            Ok(buf)
        });

        /*
        methods.add_meta_method(MetaMethod::Index, |_, this, (idx,): (i64,)| {
          let tmp_idx: usize = if idx.is_positive() { idx as usize - 1 } else { this.len() - (idx*-1) as usize };

          let res = match this.get(tmp_idx) {
            Some(element) => Some(AnyUserData::wrap(H::new(element.to_owned()))),
            None => None,
          };

          Ok(res)
        });
        */
    }
}
