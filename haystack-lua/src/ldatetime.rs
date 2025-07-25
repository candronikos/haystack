use std::fmt::Write;

use crate::{H, HError, LuaFloat};
use haystack_types::h_datetime::HDateTime;
use haystack_types::io::write::ZincWriter;
use mlua::prelude::*;
use mlua::{Function, MetaMethod, Table, UserData};

impl<'a: 'static> UserData for H<HDateTime> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut buf = String::new();
            write!(buf, "{}", ZincWriter::new(this.get_ref())).unwrap();
            Ok(buf)
        });

        methods.add_method("timestamp", |lua, this, ()| {
            let globals = lua.globals();
            let os = globals.get::<Table>("os")?;
            let time = os.get::<Function>("time")?;

            let args = lua.create_table()?;
            args.set("year", this.year())?;
            args.set("month", this.month())?;
            args.set("day", this.day())?;
            args.set("hour", this.hour())?;
            args.set("min", this.minute())?;
            args.set("sec", this.second())?;
            args.set("isdst", this.is_dst())?;

            time.call::<LuaFloat>(args)
        });

        methods.add_method("date", |_, this, ()| {
            Ok(H::new(
                this.get_ref().date().or_else(|err| Err(HError { err }))?,
            ))
        });

        methods.add_method("time", |_, this, ()| {
            Ok(H::new(
                this.get_ref().time().or_else(|err| Err(HError { err }))?,
            ))
        });

        methods.add_method("timezone", |_, this, ()| {
            Ok(this.get_ref().tz().to_string())
        });
    }
}
