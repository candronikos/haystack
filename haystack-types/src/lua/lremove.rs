use mlua::prelude::*;
use mlua::{Error as LuaError, Lua, MetaMethod, Result as LuaResult, UserData, Value};

use crate::h_remove::HRemove;
use crate::lua::H;

impl LuaUserData for H<HRemove> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok("-"));

        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: Value| match other {
            LuaValue::UserData(ud) => {
                if let Ok(_) = ud.borrow::<HRemove>() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        });
    }
}
