// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use ClockTime;
use Message;
use Object;
use ffi;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Bus(Object<ffi::GstBus, ffi::GstBusClass>): Object;

    match fn {
        get_type => || ffi::gst_bus_get_type(),
    }
}

impl Bus {
    pub fn new() -> Bus {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_bus_new())
        }
    }

    pub fn add_signal_watch(&self) {
        unsafe {
            ffi::gst_bus_add_signal_watch(self.to_glib_none().0);
        }
    }

    pub fn add_signal_watch_full(&self, priority: i32) {
        unsafe {
            ffi::gst_bus_add_signal_watch_full(self.to_glib_none().0, priority);
        }
    }

    //pub fn add_watch<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/BusFunc, user_data: P) -> u32 {
    //    unsafe { TODO: call ffi::gst_bus_add_watch() }
    //}

    //pub fn add_watch_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, priority: i32, func: /*Unknown conversion*//*Unimplemented*/BusFunc, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> u32 {
    //    unsafe { TODO: call ffi::gst_bus_add_watch_full() }
    //}

    //pub fn async_signal_func<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, message: &Message, data: P) -> bool {
    //    unsafe { TODO: call ffi::gst_bus_async_signal_func() }
    //}

    pub fn disable_sync_message_emission(&self) {
        unsafe {
            ffi::gst_bus_disable_sync_message_emission(self.to_glib_none().0);
        }
    }

    pub fn enable_sync_message_emission(&self) {
        unsafe {
            ffi::gst_bus_enable_sync_message_emission(self.to_glib_none().0);
        }
    }

    pub fn have_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_bus_have_pending(self.to_glib_none().0))
        }
    }

    pub fn peek(&self) -> Option<Message> {
        unsafe {
            from_glib_full(ffi::gst_bus_peek(self.to_glib_none().0))
        }
    }

    pub fn pop(&self) -> Option<Message> {
        unsafe {
            from_glib_full(ffi::gst_bus_pop(self.to_glib_none().0))
        }
    }

    pub fn post(&self, message: &Message) -> bool {
        unsafe {
            from_glib(ffi::gst_bus_post(self.to_glib_none().0, message.to_glib_full()))
        }
    }

    pub fn remove_signal_watch(&self) {
        unsafe {
            ffi::gst_bus_remove_signal_watch(self.to_glib_none().0);
        }
    }

    pub fn set_flushing(&self, flushing: bool) {
        unsafe {
            ffi::gst_bus_set_flushing(self.to_glib_none().0, flushing.to_glib());
        }
    }

    //pub fn set_sync_handler<'a, P: Into<Option<&'a /*Unimplemented*/BusSyncHandler>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_bus_set_sync_handler() }
    //}

    //pub fn sync_signal_handler<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, message: &Message, data: P) -> BusSyncReply {
    //    unsafe { TODO: call ffi::gst_bus_sync_signal_handler() }
    //}

    pub fn timed_pop(&self, timeout: ClockTime) -> Option<Message> {
        unsafe {
            from_glib_full(ffi::gst_bus_timed_pop(self.to_glib_none().0, timeout.to_glib()))
        }
    }

    pub fn connect_message<F: Fn(&Bus, &Message) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Bus, &Message) + Send + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "message",
                transmute(message_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_sync_message<F: Fn(&Bus, &Message) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Bus, &Message) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "sync-message",
                transmute(sync_message_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_enable_async_notify<F: Fn(&Bus) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Bus) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::enable-async",
                transmute(notify_enable_async_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Bus {}
unsafe impl Sync for Bus {}

unsafe extern "C" fn message_trampoline(this: *mut ffi::GstBus, message: *mut ffi::GstMessage, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Bus, &Message) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(message))
}

unsafe extern "C" fn sync_message_trampoline(this: *mut ffi::GstBus, message: *mut ffi::GstMessage, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Bus, &Message) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(message))
}

unsafe extern "C" fn notify_enable_async_trampoline(this: *mut ffi::GstBus, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Bus) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
