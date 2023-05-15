use crate::public::*;
use once_cell::sync::Lazy;
pub use std::{
    collections::hash_map::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
    sync::{Arc, Mutex},
    thread::spawn,
};

pub enum Bind {
    NormalBind(BindHandler),
    BlockBind(BlockBindHandler),
    BlockableBind(BlockableBindHandler),
    MouseMoveBind(MouseMoveBindHandler),
    MouseMoveBlockBind(MouseMoveBlockBindHandler),
    MouseMoveBlockableBind(MouseMoveBlockableBindHandler),
}

pub type BindHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockBindHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockableBindHandler = Arc<dyn Fn() -> BlockInput + Send + Sync + 'static>;
pub type MouseMoveBindHandler = Arc<dyn Fn(i32, i32) + Send + Sync + 'static>;
pub type MouseMoveBlockBindHandler = Arc<dyn Fn(i32, i32) + Send + Sync + 'static>;
pub type MouseMoveBlockableBindHandler = Arc<dyn Fn(i32, i32) -> BlockInput + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, Bind>;
pub type MouseBindMap = HashMap<MouseButton, Bind>;

pub static KEYBD_BINDS: Lazy<Mutex<KeybdBindMap>> = Lazy::new(|| Mutex::new(KeybdBindMap::new()));
pub static MOUSE_BINDS: Lazy<Mutex<MouseBindMap>> = Lazy::new(|| Mutex::new(MouseBindMap::new()));
