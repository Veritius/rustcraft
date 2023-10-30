pub mod namespace;
pub mod blocks;

use namespace::id::Identifier;
/// The engine's reserved content package name.
static ENGINE: Identifier = Identifier::StaticStr("engine");