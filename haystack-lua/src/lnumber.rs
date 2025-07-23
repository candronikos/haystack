use haystack_types::io::write::ZincWriter;
use mlua::prelude::*;
use std::fmt::Write;

use crate::{H, LuaFloat};
use haystack_types::h_number::HNumber;

impl LuaUserData for H<HNumber<LuaFloat>> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| Ok(this.val()));
        fields.add_field_method_get("unit", |_, this| {
            Ok(this.unit().as_ref().map(|u| u.as_str().to_owned()))
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            let mut buf = String::new();
            write!(buf, "{}", ZincWriter::new(this.get_ref())).unwrap();
            Ok(buf)
        });

        methods.add_method("tonumber", |_, this, ()| Ok(this.val()));

        methods.add_meta_method(LuaMetaMethod::Eq, |_, _this, other: LuaValue| match other {
            LuaValue::UserData(ud) => {
                if let Ok(_) = ud.borrow::<Self>() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        });
    }
}
