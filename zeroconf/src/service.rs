//! Trait definition for cross-platform service.

use crate::{EventLoop, NetworkInterface, Result, ServiceType, TxtRecord};

/// Interface for interacting with underlying mDNS service implementation registration
/// capabilities.
pub trait TMdnsService<Context: Sized = DefaultServiceContext> {
    /// Creates a new `MdnsService` with the specified `ServiceType` (e.g. `_http._tcp`) and `port`.
    fn new(service_type: ServiceType, port: u16) -> Self;

    /// Sets the name to register this service under.
    fn set_name(&mut self, name: &str);

    /// Returns the name to register this service under. In some cases, the name of the service
    /// may be auto-assigned, in which case in may not be available until after registration.
    fn name(&self) -> Option<&str>;

    /// Sets the network interface to bind this service to.
    ///
    /// Most applications will want to use the default value `NetworkInterface::Unspec` to bind to
    /// all available interfaces.
    fn set_network_interface(&mut self, interface: NetworkInterface);

    /// Returns the network interface to bind this service to.
    fn network_interface(&self) -> NetworkInterface;

    /// Sets the domain on which to advertise the service.
    ///
    /// Most applications will want to use the default value of `ptr::null()` to register to the
    /// default domain.
    fn set_domain(&mut self, domain: &str);

    /// Returns the domain on which to advertise the service.
    fn domain(&self) -> Option<&str>;

    /// Sets the SRV target host name.
    ///
    /// Most applications will want to use the default value of `ptr::null()` to use the machine's
    /// default host name.
    fn set_host(&mut self, _host: &str);

    /// Returns the SRV target host name.
    fn host(&self) -> Option<&str>;

    /// Sets the optional `TxtRecord` to register this service with.
    fn set_txt_record(&mut self, txt_record: TxtRecord);

    /// Returns the optional `TxtRecord` to register this service with.
    fn txt_record(&self) -> Option<&TxtRecord>;

    /// Sets the [`ServiceRegisteredCallback`] that is invoked when the service has been
    /// registered.
    ///
    /// [`ServiceRegisteredCallback`]: ../type.ServiceRegisteredCallback.html
    fn set_registered_callback(&mut self, registered_callback: Box<ServiceRegisteredCallback>);

    /// Sets the optional user context to pass through to the callback. This is useful if you need
    /// to share state between pre and post-callback. The context type must implement `Any`.
    fn set_context(&mut self, context: Box<Context>);

    /// Returns the optional user context.
    // fn context(&self) -> Option<Ref<Context>>;

    /// Registers and start's the service. Returns an `EventLoop` which can be called to keep
    /// the service alive.
    fn register(&mut self) -> Result<EventLoop>;
}

pub struct DefaultServiceContext;

/// Callback invoked from [`MdnsService`] once it has successfully registered.
///
/// # Arguments
/// * `service` - The service information that was registered
/// * `context` - The optional user context passed through
///
/// [`MdnsService`]: type.MdnsService.html
pub type ServiceRegisteredCallback<Context = DefaultServiceContext> =
    dyn Fn(Result<ServiceRegistration>, Option<&mut Context>);

/// Represents a registration event for a [`MdnsService`].
///
/// [`MdnsService`]: type.MdnsService.html
#[derive(Builder, BuilderDelegate, Debug, Getters, Clone, Default, PartialEq, Eq)]
pub struct ServiceRegistration {
    name: String,
    service_type: ServiceType,
    domain: String,
}
