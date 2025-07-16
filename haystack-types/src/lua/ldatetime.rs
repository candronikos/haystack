use std::os;

use crate::lua::{H, LuaFloat};
use crate::{HVal, NumTrait, h_datetime::HDateTime, io};
use mlua::prelude::*;
use mlua::{
    Table, Function, MetaMethod, UserData,
};

impl<'a: 'static> UserData for H<HDateTime> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            let mut buf = String::new();
            HVal::<LuaFloat>::to_zinc(this.get_ref(), &mut buf).unwrap();
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

        methods.add_method("date", |_, this, ()| Ok(H::new(this.get_ref().date())));

        methods.add_method("time", |_, this, ()| Ok(H::new(this.get_ref().time())));

        methods.add_method("timezone", |_, this, ()| Ok(this.get_ref().tz().to_string()));
    }
}