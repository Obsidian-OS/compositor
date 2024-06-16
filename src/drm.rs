use std::path::Path;

use smithay::{
    backend::{drm::{DrmDevice, DrmDeviceFd, DrmNode}, session::{libseat::LibSeatSession, Session}, udev::UdevBackend},
    reexports::{calloop::{EventLoop, LoopHandle}, rustix::fs::OFlags},
    utils::DeviceFd
};

// use smithay_drm_extras::drm_scanner::DrmScanner;

use crate::CalloopData;

pub struct DrmState<'eventloop> {
    session: LibSeatSession,
    event_loop: LoopHandle<'eventloop, CalloopData>
}

pub fn init_drm(event_loop: &mut EventLoop<CalloopData>, data: &mut CalloopData, ) -> Result<(), Box<dyn std::error::Error>> {

    let (session, notifier) = LibSeatSession::new()?;

    event_loop.handle().insert_source(notifier, |_,_,_| {})?;

    let udev = UdevBackend::new(session.seat())?;
    let mut state = DrmState {
        session,
        event_loop: event_loop.handle()
    };

    for (id, path) in udev.device_list() {
        if let Ok(dev) = DrmNode::from_dev_id(id) {
            state.add_device(dev, path); 
        }
    }

    return Ok(())
}

impl<'eventloop> DrmState<'eventloop> {
    pub fn add_device<Dev: AsRef<Path>>(&mut self, node: DrmNode, dev: Dev) {
        let fd = self.session
            .open(dev.as_ref(), OFlags::RDWR | OFlags::CLOEXEC | OFlags::NOCTTY | OFlags::NONBLOCK)
            .expect("Failed to open device");

        let (dev, notifier) = DrmDevice::new(DrmDeviceFd::new(DeviceFd::from(fd)), false)
            .expect("Failed to cast to native device");
    
        self.event_loop
            .insert_source(notifier, |_,_,_| {})
            .expect("Failed to insert into event loop");

        // TODO: Scan for connect/disconnect events
    }
}
