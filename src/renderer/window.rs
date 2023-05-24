use std::collections::HashMap;
use winit::window;

pub struct Window {
    pub inner: window::Window,
    pub render_surface: vello::util::RenderSurface,
}

pub type WindowId = u32;

pub struct Windows {
    inner: HashMap<WindowId, Window>,
    last_id: WindowId,
}

impl Windows {
    pub fn new(primary_window: Window) -> (Self, WindowId) {
        const PRIMARY_WINDOW_ID: WindowId = 0;
        let mut inner = HashMap::with_capacity(1);
        inner.insert(PRIMARY_WINDOW_ID, primary_window);
        let s = Self {
            inner,
            last_id: PRIMARY_WINDOW_ID,
        };
        (s, PRIMARY_WINDOW_ID)
    }

    pub fn add(&mut self, window: Window) -> WindowId {
        let id = self.last_id + 1;
        self.last_id = id;
        self.inner.insert(id, window);
        id
    }

    pub fn get(&self, k: &WindowId) -> Option<&Window> {
        self.inner.get(k)
    }

    pub fn get_mut(&mut self, k: &WindowId) -> Option<&mut Window> {
        self.inner.get_mut(k)
    }

    pub fn remove(&mut self, k: &WindowId) -> Option<Window> {
        self.inner.remove(k)
    }
}
