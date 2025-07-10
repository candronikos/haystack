use mlua::prelude::*;

use crate::h_marker::HMarker;
use crate::lua::H;

impl LuaUserData for H<HMarker> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, _this, ()| Ok("M"));

        methods.add_meta_method(LuaMetaMethod::Eq, |_, _this, other: LuaValue| match other {
            LuaValue::UserData(ud) => {
                if let Ok(_) = ud.borrow::<H<HMarker>>() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        });
    }
}
