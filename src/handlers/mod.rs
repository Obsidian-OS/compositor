mod compositor;
mod xdg_shell;

use crate::Smallvil;

//
// Wl Seat
//

use smithay::{
    wayland::output::OutputHandler,
    reexports::wayland_server::Resource,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    input::SeatState,
    input::SeatHandler,
    input::Seat,
    delegate_seat,
    delegate_output,
    delegate_data_device,
    wayland::selection::data_device::ClientDndGrabHandler,
    wayland::selection::data_device::DataDeviceHandler,
    wayland::selection::data_device::DataDeviceState,
    wayland::selection::data_device::ServerDndGrabHandler,
    wayland::selection::data_device::set_data_device_focus,
    wayland::selection::SelectionHandler
};

impl SeatHandler for Smallvil {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Smallvil> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, seat: &Seat<Self>, focused: Option<&WlSurface>) {
        let dh = &self.display_handle;
        let client = focused.and_then(|s| dh.get_client(s.id()).ok());
        set_data_device_focus(dh, seat, client);
    }

    fn cursor_image(
        &mut self, _seat: &Seat<Self>, _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
}

delegate_seat!(Smallvil);

//
// Wl Data Device
//

impl SelectionHandler for Smallvil {
    type SelectionUserData = ();
}

impl DataDeviceHandler for Smallvil {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

impl ClientDndGrabHandler for Smallvil {}
impl ServerDndGrabHandler for Smallvil {}

delegate_data_device!(Smallvil);

//
// Wl Output & Xdg Output
//

impl OutputHandler for Smallvil {}
delegate_output!(Smallvil);
