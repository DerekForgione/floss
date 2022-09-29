#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod task;
mod ui_extensions;
#[cfg(debug_assertions)]
mod debug;
pub use app::FlossApp;
