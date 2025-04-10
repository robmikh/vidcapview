use std::sync::atomic::{AtomicI32, Ordering};
use windows::{
    Win32::{
        Foundation::HWND,
        UI::Input::KeyboardAndMouse::{HOT_KEY_MODIFIERS, RegisterHotKey, UnregisterHotKey},
    },
    core::Result,
};

static HOT_KEY_ID: AtomicI32 = AtomicI32::new(0);

pub struct HotKey {
    window: HWND,
    id: i32,
}

impl HotKey {
    pub fn new(window: HWND, modifiers: HOT_KEY_MODIFIERS, key: u32) -> Result<Self> {
        let id = HOT_KEY_ID.fetch_add(1, Ordering::SeqCst) + 1;
        unsafe {
            RegisterHotKey(Some(window), id, modifiers, key)?;
        }
        Ok(Self { window, id })
    }
}

impl Drop for HotKey {
    fn drop(&mut self) {
        let _ = unsafe { UnregisterHotKey(Some(self.window), self.id) };
    }
}
