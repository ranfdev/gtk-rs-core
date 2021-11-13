// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Cancellable;
use crate::Credentials;
use crate::DatagramBased;
use crate::InetAddress;
use crate::Initable;
use crate::SocketAddress;
use crate::SocketConnection;
use crate::SocketFamily;
use crate::SocketProtocol;
use crate::SocketType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ObjectExt;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GSocket")]
    pub struct Socket(Object<ffi::GSocket, ffi::GSocketClass>) @implements DatagramBased, Initable;

    match fn {
        type_ => || ffi::g_socket_get_type(),
    }
}

impl Socket {
    #[doc(alias = "g_socket_new")]
    pub fn new(
        family: SocketFamily,
        type_: SocketType,
        protocol: SocketProtocol,
    ) -> Result<Socket, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_new(
                family.into_glib(),
                type_.into_glib(),
                protocol.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl glib::SendUnique for Socket {
    fn is_unique(&self) -> bool {
        self.ref_count() == 1
    }
}

impl Socket {
    pub const NONE: Option<&'static Socket> = None;
}

pub trait SocketExt: 'static {
    #[doc(alias = "g_socket_accept")]
    fn accept(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<Socket, glib::Error>;

    #[doc(alias = "g_socket_bind")]
    fn bind(&self, address: &impl IsA<SocketAddress>, allow_reuse: bool)
        -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_check_connect_result")]
    fn check_connect_result(&self) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_close")]
    fn close(&self) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_condition_timed_wait")]
    fn condition_timed_wait(
        &self,
        condition: glib::IOCondition,
        timeout_us: i64,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_condition_wait")]
    fn condition_wait(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_connect")]
    fn connect(
        &self,
        address: &impl IsA<SocketAddress>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_connection_factory_create_connection")]
    fn connection_factory_create_connection(&self) -> SocketConnection;

    #[doc(alias = "g_socket_get_available_bytes")]
    #[doc(alias = "get_available_bytes")]
    fn available_bytes(&self) -> isize;

    #[doc(alias = "g_socket_get_blocking")]
    #[doc(alias = "get_blocking")]
    fn is_blocking(&self) -> bool;

    #[doc(alias = "g_socket_get_broadcast")]
    #[doc(alias = "get_broadcast")]
    fn is_broadcast(&self) -> bool;

    #[doc(alias = "g_socket_get_credentials")]
    #[doc(alias = "get_credentials")]
    fn credentials(&self) -> Result<Credentials, glib::Error>;

    #[doc(alias = "g_socket_get_family")]
    #[doc(alias = "get_family")]
    fn family(&self) -> SocketFamily;

    #[doc(alias = "g_socket_get_keepalive")]
    #[doc(alias = "get_keepalive")]
    fn is_keepalive(&self) -> bool;

    #[doc(alias = "g_socket_get_listen_backlog")]
    #[doc(alias = "get_listen_backlog")]
    fn listen_backlog(&self) -> i32;

    #[doc(alias = "g_socket_get_local_address")]
    #[doc(alias = "get_local_address")]
    fn local_address(&self) -> Result<SocketAddress, glib::Error>;

    #[doc(alias = "g_socket_get_multicast_loopback")]
    #[doc(alias = "get_multicast_loopback")]
    fn is_multicast_loopback(&self) -> bool;

    #[doc(alias = "g_socket_get_multicast_ttl")]
    #[doc(alias = "get_multicast_ttl")]
    fn multicast_ttl(&self) -> u32;

    #[doc(alias = "g_socket_get_option")]
    #[doc(alias = "get_option")]
    fn option(&self, level: i32, optname: i32) -> Result<i32, glib::Error>;

    #[doc(alias = "g_socket_get_protocol")]
    #[doc(alias = "get_protocol")]
    fn protocol(&self) -> SocketProtocol;

    #[doc(alias = "g_socket_get_remote_address")]
    #[doc(alias = "get_remote_address")]
    fn remote_address(&self) -> Result<SocketAddress, glib::Error>;

    #[doc(alias = "g_socket_get_socket_type")]
    #[doc(alias = "get_socket_type")]
    fn socket_type(&self) -> SocketType;

    #[doc(alias = "g_socket_get_timeout")]
    #[doc(alias = "get_timeout")]
    fn timeout(&self) -> u32;

    #[doc(alias = "g_socket_get_ttl")]
    #[doc(alias = "get_ttl")]
    fn ttl(&self) -> u32;

    #[doc(alias = "g_socket_is_closed")]
    fn is_closed(&self) -> bool;

    #[doc(alias = "g_socket_is_connected")]
    fn is_connected(&self) -> bool;

    #[doc(alias = "g_socket_join_multicast_group")]
    fn join_multicast_group(
        &self,
        group: &impl IsA<InetAddress>,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "g_socket_join_multicast_group_ssm")]
    fn join_multicast_group_ssm(
        &self,
        group: &impl IsA<InetAddress>,
        source_specific: Option<&impl IsA<InetAddress>>,
        iface: Option<&str>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_leave_multicast_group")]
    fn leave_multicast_group(
        &self,
        group: &impl IsA<InetAddress>,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "g_socket_leave_multicast_group_ssm")]
    fn leave_multicast_group_ssm(
        &self,
        group: &impl IsA<InetAddress>,
        source_specific: Option<&impl IsA<InetAddress>>,
        iface: Option<&str>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_listen")]
    fn listen(&self) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_set_blocking")]
    fn set_blocking(&self, blocking: bool);

    #[doc(alias = "g_socket_set_broadcast")]
    fn set_broadcast(&self, broadcast: bool);

    #[doc(alias = "g_socket_set_keepalive")]
    fn set_keepalive(&self, keepalive: bool);

    #[doc(alias = "g_socket_set_listen_backlog")]
    fn set_listen_backlog(&self, backlog: i32);

    #[doc(alias = "g_socket_set_multicast_loopback")]
    fn set_multicast_loopback(&self, loopback: bool);

    #[doc(alias = "g_socket_set_multicast_ttl")]
    fn set_multicast_ttl(&self, ttl: u32);

    #[doc(alias = "g_socket_set_option")]
    fn set_option(&self, level: i32, optname: i32, value: i32) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_set_timeout")]
    fn set_timeout(&self, timeout: u32);

    #[doc(alias = "g_socket_set_ttl")]
    fn set_ttl(&self, ttl: u32);

    #[doc(alias = "g_socket_shutdown")]
    fn shutdown(&self, shutdown_read: bool, shutdown_write: bool) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_speaks_ipv4")]
    fn speaks_ipv4(&self) -> bool;

    #[doc(alias = "type")]
    fn type_(&self) -> SocketType;

    #[doc(alias = "blocking")]
    fn connect_blocking_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "broadcast")]
    fn connect_broadcast_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "keepalive")]
    fn connect_keepalive_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "listen-backlog")]
    fn connect_listen_backlog_notify<F: Fn(&Self) + Send + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[doc(alias = "local-address")]
    fn connect_local_address_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "multicast-loopback")]
    fn connect_multicast_loopback_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "multicast-ttl")]
    fn connect_multicast_ttl_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "remote-address")]
    fn connect_remote_address_notify<F: Fn(&Self) + Send + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[doc(alias = "timeout")]
    fn connect_timeout_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ttl")]
    fn connect_ttl_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Socket>> SocketExt for O {
    fn accept(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<Socket, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_accept(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn bind(
        &self,
        address: &impl IsA<SocketAddress>,
        allow_reuse: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_bind(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                allow_reuse.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn check_connect_result(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_check_connect_result(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_close(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn condition_timed_wait(
        &self,
        condition: glib::IOCondition,
        timeout_us: i64,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_condition_timed_wait(
                self.as_ref().to_glib_none().0,
                condition.into_glib(),
                timeout_us,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn condition_wait(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_condition_wait(
                self.as_ref().to_glib_none().0,
                condition.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect(
        &self,
        address: &impl IsA<SocketAddress>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_connect(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connection_factory_create_connection(&self) -> SocketConnection {
        unsafe {
            from_glib_full(ffi::g_socket_connection_factory_create_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn available_bytes(&self) -> isize {
        unsafe { ffi::g_socket_get_available_bytes(self.as_ref().to_glib_none().0) }
    }

    fn is_blocking(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_get_blocking(self.as_ref().to_glib_none().0)) }
    }

    fn is_broadcast(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_get_broadcast(self.as_ref().to_glib_none().0)) }
    }

    fn credentials(&self) -> Result<Credentials, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_credentials(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn family(&self) -> SocketFamily {
        unsafe { from_glib(ffi::g_socket_get_family(self.as_ref().to_glib_none().0)) }
    }

    fn is_keepalive(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_get_keepalive(self.as_ref().to_glib_none().0)) }
    }

    fn listen_backlog(&self) -> i32 {
        unsafe { ffi::g_socket_get_listen_backlog(self.as_ref().to_glib_none().0) }
    }

    fn local_address(&self) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_local_address(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn is_multicast_loopback(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_get_multicast_loopback(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn multicast_ttl(&self) -> u32 {
        unsafe { ffi::g_socket_get_multicast_ttl(self.as_ref().to_glib_none().0) }
    }

    fn option(&self, level: i32, optname: i32) -> Result<i32, glib::Error> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_get_option(
                self.as_ref().to_glib_none().0,
                level,
                optname,
                value.as_mut_ptr(),
                &mut error,
            );
            let value = value.assume_init();
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn protocol(&self) -> SocketProtocol {
        unsafe { from_glib(ffi::g_socket_get_protocol(self.as_ref().to_glib_none().0)) }
    }

    fn remote_address(&self) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_remote_address(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn socket_type(&self) -> SocketType {
        unsafe {
            from_glib(ffi::g_socket_get_socket_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn timeout(&self) -> u32 {
        unsafe { ffi::g_socket_get_timeout(self.as_ref().to_glib_none().0) }
    }

    fn ttl(&self) -> u32 {
        unsafe { ffi::g_socket_get_ttl(self.as_ref().to_glib_none().0) }
    }

    fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_is_closed(self.as_ref().to_glib_none().0)) }
    }

    fn is_connected(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_is_connected(self.as_ref().to_glib_none().0)) }
    }

    fn join_multicast_group(
        &self,
        group: &impl IsA<InetAddress>,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_join_multicast_group(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.into_glib(),
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn join_multicast_group_ssm(
        &self,
        group: &impl IsA<InetAddress>,
        source_specific: Option<&impl IsA<InetAddress>>,
        iface: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_join_multicast_group_ssm(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.map(|p| p.as_ref()).to_glib_none().0,
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn leave_multicast_group(
        &self,
        group: &impl IsA<InetAddress>,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_leave_multicast_group(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.into_glib(),
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn leave_multicast_group_ssm(
        &self,
        group: &impl IsA<InetAddress>,
        source_specific: Option<&impl IsA<InetAddress>>,
        iface: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_leave_multicast_group_ssm(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.map(|p| p.as_ref()).to_glib_none().0,
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn listen(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_listen(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_blocking(&self, blocking: bool) {
        unsafe {
            ffi::g_socket_set_blocking(self.as_ref().to_glib_none().0, blocking.into_glib());
        }
    }

    fn set_broadcast(&self, broadcast: bool) {
        unsafe {
            ffi::g_socket_set_broadcast(self.as_ref().to_glib_none().0, broadcast.into_glib());
        }
    }

    fn set_keepalive(&self, keepalive: bool) {
        unsafe {
            ffi::g_socket_set_keepalive(self.as_ref().to_glib_none().0, keepalive.into_glib());
        }
    }

    fn set_listen_backlog(&self, backlog: i32) {
        unsafe {
            ffi::g_socket_set_listen_backlog(self.as_ref().to_glib_none().0, backlog);
        }
    }

    fn set_multicast_loopback(&self, loopback: bool) {
        unsafe {
            ffi::g_socket_set_multicast_loopback(
                self.as_ref().to_glib_none().0,
                loopback.into_glib(),
            );
        }
    }

    fn set_multicast_ttl(&self, ttl: u32) {
        unsafe {
            ffi::g_socket_set_multicast_ttl(self.as_ref().to_glib_none().0, ttl);
        }
    }

    fn set_option(&self, level: i32, optname: i32, value: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_set_option(
                self.as_ref().to_glib_none().0,
                level,
                optname,
                value,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::g_socket_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    fn set_ttl(&self, ttl: u32) {
        unsafe {
            ffi::g_socket_set_ttl(self.as_ref().to_glib_none().0, ttl);
        }
    }

    fn shutdown(&self, shutdown_read: bool, shutdown_write: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_shutdown(
                self.as_ref().to_glib_none().0,
                shutdown_read.into_glib(),
                shutdown_write.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn speaks_ipv4(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_speaks_ipv4(self.as_ref().to_glib_none().0)) }
    }

    fn type_(&self) -> SocketType {
        glib::ObjectExt::property(self.as_ref(), "type")
    }

    fn connect_blocking_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_blocking_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::blocking\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_blocking_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_broadcast_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_broadcast_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::broadcast\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_broadcast_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_keepalive_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_keepalive_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::keepalive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_keepalive_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_listen_backlog_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_listen_backlog_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::listen-backlog\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_listen_backlog_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_local_address_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_address_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_multicast_loopback_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_multicast_loopback_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::multicast-loopback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multicast_loopback_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_multicast_ttl_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_multicast_ttl_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::multicast-ttl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multicast_ttl_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_remote_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_remote_address_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::remote-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_remote_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_timeout_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<
            P: IsA<Socket>,
            F: Fn(&P) + Send + 'static,
        >(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ttl_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ttl_trampoline<P: IsA<Socket>, F: Fn(&P) + Send + 'static>(
            this: *mut ffi::GSocket,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Socket::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ttl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ttl_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Socket")
    }
}
