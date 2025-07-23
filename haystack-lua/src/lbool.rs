use mlua::prelude::*;
use mlua::{MetaMethod, Value};

use crate::H;
use haystack_types::h_bool::HBool;

impl LuaUserData for H<HBool> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            if this.0 { Ok("true") } else { Ok("false") }
        });

        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: Value| match other {
            LuaValue::UserData(ud) => {
                if let Ok(b) = ud.borrow::<HBool>() {
                    Ok(b.0 == this.0)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        });
    }
}
