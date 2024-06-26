use crate::{
    grabs::resize_grab,
    state::ClientState,
    Smallvil
};

use smithay::{
    wayland::shm::ShmHandler,
    wayland::compositor::CompositorState,
    wayland::compositor::CompositorHandler,
    wayland::compositor::CompositorClientState,
    wayland::compositor::is_sync_subsurface,
    wayland::compositor::get_parent,
    wayland::buffer::BufferHandler,
    reexports::wayland_server::Client,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    reexports::wayland_server::protocol::wl_buffer,
    delegate_shm,
    delegate_compositor,
    backend::renderer::utils::on_commit_buffer_handler,
    wayland::shm::ShmState
};

use super::xdg_shell;

impl CompositorHandler for Smallvil {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }
            if let Some(window) = self
                .space
                .elements()
                .find(|w| w.toplevel().unwrap().wl_surface() == &root)
            {
                window.on_commit();
            }
        };

        xdg_shell::handle_commit(&mut self.popups, &self.space, surface);
        resize_grab::handle_commit(&mut self.space, surface);
    }
}

impl BufferHandler for Smallvil {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}

impl ShmHandler for Smallvil {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

delegate_compositor!(Smallvil);delegate_shm!(Smallvil);
