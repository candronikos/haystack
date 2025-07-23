use mlua::prelude::*;
use mlua::{MetaMethod, Value};

use crate::H;
use haystack_types::h_null::HNull;

impl LuaUserData for H<HNull> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, _this, ()| Ok("null"));

        methods.add_meta_method(LuaMetaMethod::Eq, |_, _this, other: Value| match other {
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
