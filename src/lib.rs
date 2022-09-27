#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod tasker;
mod ui_extensions;
#[cfg(debug_assertions)]
mod debug;
pub use app::FlossApp;
