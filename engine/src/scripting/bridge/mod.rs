mod stdlib;

/// A bridge between types from foreign crates and Lua.
/// Implements `Deref` for `T`, most standard library traits where `T` does, and `FromLua` and `IntoLua` for certain types.
#[repr(transparent)]
pub struct Bridge<T>(T);

impl<T> std::ops::Deref for Bridge<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Bridge<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}