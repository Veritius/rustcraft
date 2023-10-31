/// A unique, short numerical identifier for a block, used for dense storage of voxel information.
/// `BlockId`s are useless by themselves, and are used to access information stored in the [`BlockRegistry`](super::registry::BlockRegistry).
/// 
/// A `BlockId` normally has `65536` aka `2^16` possible values.
/// With the `big_ids` feature flag, it has `16777216` aka `2^24` possible values.
/// Accordingly, the size in memory goes from 2 bytes to 3 bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(internal::BlockIdInner);

impl mlua::FromLua<'_> for BlockId {
    fn from_lua(value: mlua::Value<'_>, lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        Ok(Self(internal::BlockIdInner::from_lua(value, lua)?))
    }
}

impl mlua::IntoLua<'_> for BlockId {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        self.0.into_lua(lua)
    }
}

#[cfg(not(feature="big_ids"))]
mod internal {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub(super) struct BlockIdInner(u16);
    
    impl From<u16> for BlockIdInner {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    
    impl Into<u16> for BlockIdInner {
        fn into(self) -> u16 {
            self.0
        }
    }

    impl mlua::FromLua<'_> for BlockIdInner {
        fn from_lua(value: mlua::Value<'_>, lua: &'_ mlua::Lua) -> mlua::Result<Self> {
            match value {
                mlua::Value::Integer(i) => {
                    let v = i.abs() as u32;
                    if v > 2u32.pow(16) {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: "integer",
                            to: "blockid",
                            message: Some(format!("Passed integer was an unrepresentable value: {v}")),
                        });
                    }
                    Ok(Self(v.try_into().unwrap()))
                },
                _ => {
                    Err(mlua::Error::FromLuaConversionError {
                        from: value.type_name(),
                        to: "blockid",
                        message: None,
                    })
                }
            }   
        }
    }

    impl mlua::IntoLua<'_> for BlockIdInner {
        fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
            self.0.into_lua(lua)
        }
    }
}

#[cfg(feature="big_ids")]
mod internal {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub(super) struct BlockIdInner([u8;3]);

    impl TryFrom<u32> for BlockIdInner {
        type Error = ();

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            if value > 2u32.pow(24) { return Err(() )}
            let bytes = value.to_be_bytes();
            Ok(Self([bytes[0], bytes[1], bytes[2]]))
        }
    }

    impl Into<u32> for BlockIdInner {
        fn into(self) -> u32 {
            u32::from_be_bytes([self.0[0], self.0[1], self.0[2], 0])
        }
    }

    impl mlua::FromLua<'_> for BlockIdInner {
        fn from_lua(value: mlua::Value<'_>, lua: &'_ mlua::Lua) -> mlua::Result<Self> {
            match value {
                mlua::Value::Integer(i) => {
                    let v = i.abs() as u32;
                    if v > 2u32.pow(24) {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: "integer",
                            to: "blockid",
                            message: Some(format!("Passed integer was an unrepresentable value: {v}")),
                        });
                    }
                    Ok(Self::try_from(v).unwrap())
                },
                _ => {
                    Err(mlua::Error::FromLuaConversionError {
                        from: value.type_name(),
                        to: "blockid",
                        message: None,
                    })
                }
            }   
        }
    }

    impl mlua::IntoLua<'_> for BlockIdInner {
        fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
            self.0.into_lua(lua)
        }
    }
}

/// Creates a sequential series of `BlockID`s.
/// Panics when the upper limit of `BlockID`s are achieved.
#[derive(Default)]
pub(super) struct BlockIdGenerator {
    #[cfg(not(feature="big_ids"))]
    state: u16,
    #[cfg(feature="big_ids")]
    state: u32,
}

impl BlockIdGenerator {
    #[cfg(not(feature="big_ids"))]
    pub fn next(&mut self) -> BlockId {
        let id = BlockId(internal::BlockIdInner::from(self.state));
        self.state = self.state.checked_add(1)
            .expect("Ran out of possible BlockId values at 2^16");
        return id
    }

    #[cfg(feature="big_ids")]
    pub fn next(&mut self) -> BlockId {
        if self.state > 2u32.pow(24) { panic!("Ran out of possible BlockId values at 2^24") }
        let id = BlockId(internal::BlockIdInner::try_from(self.state).unwrap());
        self.state += 1;
        return id
    }
}