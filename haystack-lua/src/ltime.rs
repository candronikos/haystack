use crate::{H, LuaFloat};
use haystack_types::io::write::ZincWriter;
use haystack_types::{HVal, NumTrait, h_time::HTime, io};
use mlua::prelude::*;
use mlua::{MetaMethod, UserData};
use std::fmt::Write;

impl<'a: 'static> UserData for H<HTime> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut buf = String::new();
            write!(buf, "{}", ZincWriter::new(this.get_ref())).unwrap();
            Ok(buf)
        });
    }
}
