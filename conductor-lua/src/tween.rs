use conductor::tween::Tween;
use mlua::prelude::*;

use crate::error::ConductorLuaError;

pub struct LTween(pub Tween);

impl<'lua> FromLua<'lua> for LTween {
	fn from_lua(lua_value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
		match lua_value {
			LuaValue::Number(duration) => Ok(LTween(Tween(duration))),
			LuaValue::Table(table) => Ok(LTween(Tween(table.get(1)?))),
			_ => Err(LuaError::external(ConductorLuaError::wrong_argument_type(
				"tween",
				"table or number",
			))),
		}
	}
}
