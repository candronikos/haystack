use crate::H;
use haystack_types::h_uri::HUri;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData};

impl<'a: 'static> UserData for H<HUri> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_owned_string())
        });
    }
}
