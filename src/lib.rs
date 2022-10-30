#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(rustdoc::all)]

#[macro_use]
pub mod utilities;
pub mod gatt_server;

// TODO: Better log levels.
// TODO: Custom errors instead of panics.
// TODO: Clippy.
// TODO: Builder pattern.
