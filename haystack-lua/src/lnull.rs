use mlua::prelude::*;
use mlua::{Error as LuaError, Lua, MetaMethod, Result as LuaResult, UserData, Value};

use haystack_types::h_null::HNull;
use crate::H;

impl LuaUserData for H<HNull> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok("null"));

        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: Value| match other {
            LuaValue::UserData(ud) => {
                if let Ok(_) = ud.borrow::<HNull>() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        });
    }
}
