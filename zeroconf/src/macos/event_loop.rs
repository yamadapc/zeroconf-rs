//! Event loop for running a `MdnsService` or `MdnsBrowser`.

use super::service_ref::ManagedDNSServiceRef;
use crate::event_loop::TEventLoop;
use crate::{ffi, Result};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;

#[derive(new)]
pub struct BonjourEventLoop<'a> {
    service: Rc<RefCell<ManagedDNSServiceRef>>,
    phantom: PhantomData<&'a ManagedDNSServiceRef>,
}

impl<'a> TEventLoop for BonjourEventLoop<'a> {
    /// Polls for new events.
    ///
    /// Prior to calling `ManagedDNSServiceRef::process_result()`, this function performs a unix
    /// `select()` on the underlying socket with the specified timeout. If the socket contains no
    /// new data, the blocking call is not made.
    fn poll(&self, timeout: Duration) -> Result<()> {
        let service = self.service.borrow_mut();
        let select = unsafe { ffi::macos::read_select(service.sock_fd(), timeout)? };
        if select > 0 {
            service.process_result()
        } else {
            Ok(())
        }
    }
}
