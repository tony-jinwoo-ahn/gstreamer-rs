// This file was generated by gir (a01311c+) from gir-files (???)
// DO NOT EDIT

use Bus;
use Caps;
use Clock;
use ClockTime;
use ElementFactory;
use Error;
use Event;
use Format;
use Iterator;
use Message;
use Object;
use Pad;
use PadTemplate;
use Plugin;
use SeekFlags;
use SeekType;
use State;
use StateChange;
use StateChangeReturn;
use URIType;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v1_10")]
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Element(Object<ffi::GstElement>): Object;

    match fn {
        get_type => || ffi::gst_element_get_type(),
    }
}

impl Element {
    pub fn make_from_uri<'a, P: Into<Option<&'a str>>>(type_: URIType, uri: &str, elementname: P) -> Result<Element, Error> {
        assert_initialized_main_thread!();
        let elementname = elementname.into();
        let elementname = elementname.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_element_make_from_uri(type_.to_glib(), uri.to_glib_none().0, elementname.0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn register<'a, P: Into<Option<&'a Plugin>>>(plugin: P, name: &str, rank: u32, type_: glib::types::Type) -> bool {
        assert_initialized_main_thread!();
        let plugin = plugin.into();
        let plugin = plugin.to_glib_none();
        unsafe {
            from_glib(ffi::gst_element_register(plugin.0, name.to_glib_none().0, rank, type_.to_glib()))
        }
    }

    pub fn state_change_return_get_name(state_ret: StateChangeReturn) -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_element_state_change_return_get_name(state_ret.to_glib()))
        }
    }

    pub fn state_get_name(state: State) -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_element_state_get_name(state.to_glib()))
        }
    }
}

unsafe impl Send for Element {}
unsafe impl Sync for Element {}

pub trait ElementExt {
    fn abort_state(&self);

    fn add_pad<P: IsA<Pad>>(&self, pad: &P) -> Result<(), glib::error::BoolError>;

    #[cfg(feature = "v1_10")]
    fn add_property_deep_notify_watch<'a, P: Into<Option<&'a str>>>(&self, property_name: P, include_value: bool) -> libc::c_ulong;

    #[cfg(feature = "v1_10")]
    fn add_property_notify_watch<'a, P: Into<Option<&'a str>>>(&self, property_name: P, include_value: bool) -> libc::c_ulong;

    //#[cfg(feature = "v1_10")]
    //fn call_async<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/ElementCallAsyncFunc, user_data: P, destroy_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn change_state(&self, transition: StateChange) -> StateChangeReturn;

    fn continue_state(&self, ret: StateChangeReturn) -> StateChangeReturn;

    fn create_all_pads(&self);

    fn get_base_time(&self) -> ClockTime;

    fn get_bus(&self) -> Option<Bus>;

    fn get_clock(&self) -> Option<Clock>;

    fn get_compatible_pad<'a, P: IsA<Pad>, Q: Into<Option<&'a Caps>>>(&self, pad: &P, caps: Q) -> Option<Pad>;

    fn get_compatible_pad_template(&self, compattempl: &PadTemplate) -> Option<PadTemplate>;

    //fn get_context(&self, context_type: &str) -> /*Ignored*/Option<Context>;

    //fn get_context_unlocked(&self, context_type: &str) -> /*Ignored*/Option<Context>;

    //fn get_contexts(&self) -> /*Ignored*/Vec<Context>;

    fn get_factory(&self) -> Option<ElementFactory>;

    fn get_request_pad(&self, name: &str) -> Option<Pad>;

    fn get_start_time(&self) -> ClockTime;

    fn get_state(&self, timeout: ClockTime) -> (StateChangeReturn, State, State);

    fn get_static_pad(&self, name: &str) -> Option<Pad>;

    fn is_locked_state(&self) -> bool;

    fn iterate_pads(&self) -> Option<Iterator>;

    fn iterate_sink_pads(&self) -> Option<Iterator>;

    fn iterate_src_pads(&self) -> Option<Iterator>;

    fn link<P: IsA<Element>>(&self, dest: &P) -> Result<(), glib::error::BoolError>;

    fn link_filtered<'a, P: IsA<Element>, Q: Into<Option<&'a Caps>>>(&self, dest: &P, filter: Q) -> Result<(), glib::error::BoolError>;

    //fn link_many<P: IsA<Element>>(&self, element_2: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    fn link_pads<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>>(&self, srcpadname: P, dest: &Q, destpadname: R) -> Result<(), glib::error::BoolError>;

    fn link_pads_filtered<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>, S: Into<Option<&'c Caps>>>(&self, srcpadname: P, dest: &Q, destpadname: R, filter: S) -> Result<(), glib::error::BoolError>;

    //fn link_pads_full<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>>(&self, srcpadname: P, dest: &Q, destpadname: R, flags: /*Ignored*/PadLinkCheck) -> bool;

    fn lost_state(&self);

    //fn message_full<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, type_: /*Ignored*/MessageType, domain: /*Ignored*/glib::Quark, code: i32, text: P, debug: Q, file: &str, function: &str, line: i32);

    //#[cfg(feature = "v1_10")]
    //fn message_full_with_details<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, type_: /*Ignored*/MessageType, domain: /*Ignored*/glib::Quark, code: i32, text: P, debug: Q, file: &str, function: &str, line: i32, structure: &mut Structure);

    fn no_more_pads(&self);

    fn post_message(&self, message: &Message) -> bool;

    fn provide_clock(&self) -> Option<Clock>;

    fn query_convert(&self, src_format: Format, src_val: i64, dest_format: Format) -> Option<i64>;

    fn query_duration(&self, format: Format) -> Option<i64>;

    fn query_position(&self, format: Format) -> Option<i64>;

    fn release_request_pad<P: IsA<Pad>>(&self, pad: &P);

    fn remove_pad<P: IsA<Pad>>(&self, pad: &P) -> Result<(), glib::error::BoolError>;

    #[cfg(feature = "v1_10")]
    fn remove_property_notify_watch(&self, watch_id: libc::c_ulong);

    fn request_pad<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(&self, templ: &PadTemplate, name: P, caps: Q) -> Option<Pad>;

    fn seek(&self, rate: f64, format: Format, flags: SeekFlags, start_type: SeekType, start: i64, stop_type: SeekType, stop: i64) -> Result<(), glib::error::BoolError>;

    fn seek_simple(&self, format: Format, seek_flags: SeekFlags, seek_pos: i64) -> Result<(), glib::error::BoolError>;

    fn send_event(&self, event: &Event) -> bool;

    fn set_base_time(&self, time: ClockTime);

    fn set_bus(&self, bus: &Bus);

    fn set_clock<P: IsA<Clock>>(&self, clock: &P) -> Result<(), glib::error::BoolError>;

    //fn set_context(&self, context: /*Ignored*/&mut Context);

    fn set_locked_state(&self, locked_state: bool) -> bool;

    fn set_start_time(&self, time: ClockTime);

    fn set_state(&self, state: State) -> StateChangeReturn;

    fn sync_state_with_parent(&self) -> Result<(), glib::error::BoolError>;

    fn unlink<P: IsA<Element>>(&self, dest: &P);

    //fn unlink_many<P: IsA<Element>>(&self, element_2: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn unlink_pads<P: IsA<Element>>(&self, srcpadname: &str, dest: &P, destpadname: &str);

    fn connect_no_more_pads<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> u64;

    fn connect_pad_added<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> u64;

    fn connect_pad_removed<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Element> + IsA<glib::object::Object>> ElementExt for O {
    fn abort_state(&self) {
        unsafe {
            ffi::gst_element_abort_state(self.to_glib_none().0);
        }
    }

    fn add_pad<P: IsA<Pad>>(&self, pad: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_add_pad(self.to_glib_none().0, pad.to_glib_none().0), "Failed to add pad")
        }
    }

    #[cfg(feature = "v1_10")]
    fn add_property_deep_notify_watch<'a, P: Into<Option<&'a str>>>(&self, property_name: P, include_value: bool) -> libc::c_ulong {
        let property_name = property_name.into();
        let property_name = property_name.to_glib_none();
        unsafe {
            ffi::gst_element_add_property_deep_notify_watch(self.to_glib_none().0, property_name.0, include_value.to_glib())
        }
    }

    #[cfg(feature = "v1_10")]
    fn add_property_notify_watch<'a, P: Into<Option<&'a str>>>(&self, property_name: P, include_value: bool) -> libc::c_ulong {
        let property_name = property_name.into();
        let property_name = property_name.to_glib_none();
        unsafe {
            ffi::gst_element_add_property_notify_watch(self.to_glib_none().0, property_name.0, include_value.to_glib())
        }
    }

    //#[cfg(feature = "v1_10")]
    //fn call_async<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/ElementCallAsyncFunc, user_data: P, destroy_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_element_call_async() }
    //}

    fn change_state(&self, transition: StateChange) -> StateChangeReturn {
        unsafe {
            from_glib(ffi::gst_element_change_state(self.to_glib_none().0, transition.to_glib()))
        }
    }

    fn continue_state(&self, ret: StateChangeReturn) -> StateChangeReturn {
        unsafe {
            from_glib(ffi::gst_element_continue_state(self.to_glib_none().0, ret.to_glib()))
        }
    }

    fn create_all_pads(&self) {
        unsafe {
            ffi::gst_element_create_all_pads(self.to_glib_none().0);
        }
    }

    fn get_base_time(&self) -> ClockTime {
        unsafe {
            ffi::gst_element_get_base_time(self.to_glib_none().0)
        }
    }

    fn get_bus(&self) -> Option<Bus> {
        unsafe {
            from_glib_full(ffi::gst_element_get_bus(self.to_glib_none().0))
        }
    }

    fn get_clock(&self) -> Option<Clock> {
        unsafe {
            from_glib_full(ffi::gst_element_get_clock(self.to_glib_none().0))
        }
    }

    fn get_compatible_pad<'a, P: IsA<Pad>, Q: Into<Option<&'a Caps>>>(&self, pad: &P, caps: Q) -> Option<Pad> {
        let caps = caps.into();
        let caps = caps.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_element_get_compatible_pad(self.to_glib_none().0, pad.to_glib_none().0, caps.0))
        }
    }

    fn get_compatible_pad_template(&self, compattempl: &PadTemplate) -> Option<PadTemplate> {
        unsafe {
            from_glib_none(ffi::gst_element_get_compatible_pad_template(self.to_glib_none().0, compattempl.to_glib_none().0))
        }
    }

    //fn get_context(&self, context_type: &str) -> /*Ignored*/Option<Context> {
    //    unsafe { TODO: call ffi::gst_element_get_context() }
    //}

    //fn get_context_unlocked(&self, context_type: &str) -> /*Ignored*/Option<Context> {
    //    unsafe { TODO: call ffi::gst_element_get_context_unlocked() }
    //}

    //fn get_contexts(&self) -> /*Ignored*/Vec<Context> {
    //    unsafe { TODO: call ffi::gst_element_get_contexts() }
    //}

    fn get_factory(&self) -> Option<ElementFactory> {
        unsafe {
            from_glib_none(ffi::gst_element_get_factory(self.to_glib_none().0))
        }
    }

    fn get_request_pad(&self, name: &str) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_get_request_pad(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_start_time(&self) -> ClockTime {
        unsafe {
            ffi::gst_element_get_start_time(self.to_glib_none().0)
        }
    }

    fn get_state(&self, timeout: ClockTime) -> (StateChangeReturn, State, State) {
        unsafe {
            let mut state = mem::uninitialized();
            let mut pending = mem::uninitialized();
            let ret = from_glib(ffi::gst_element_get_state(self.to_glib_none().0, &mut state, &mut pending, timeout));
            (ret, from_glib(state), from_glib(pending))
        }
    }

    fn get_static_pad(&self, name: &str) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_element_get_static_pad(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn is_locked_state(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_element_is_locked_state(self.to_glib_none().0))
        }
    }

    fn iterate_pads(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_pads(self.to_glib_none().0))
        }
    }

    fn iterate_sink_pads(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_sink_pads(self.to_glib_none().0))
        }
    }

    fn iterate_src_pads(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::gst_element_iterate_src_pads(self.to_glib_none().0))
        }
    }

    fn link<P: IsA<Element>>(&self, dest: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_link(self.to_glib_none().0, dest.to_glib_none().0), "Failed to link elements")
        }
    }

    fn link_filtered<'a, P: IsA<Element>, Q: Into<Option<&'a Caps>>>(&self, dest: &P, filter: Q) -> Result<(), glib::error::BoolError> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_link_filtered(self.to_glib_none().0, dest.to_glib_none().0, filter.0), "Failed to link elements")
        }
    }

    //fn link_many<P: IsA<Element>>(&self, element_2: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::gst_element_link_many() }
    //}

    fn link_pads<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>>(&self, srcpadname: P, dest: &Q, destpadname: R) -> Result<(), glib::error::BoolError> {
        let srcpadname = srcpadname.into();
        let srcpadname = srcpadname.to_glib_none();
        let destpadname = destpadname.into();
        let destpadname = destpadname.to_glib_none();
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_link_pads(self.to_glib_none().0, srcpadname.0, dest.to_glib_none().0, destpadname.0), "Failed to link pads")
        }
    }

    fn link_pads_filtered<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>, S: Into<Option<&'c Caps>>>(&self, srcpadname: P, dest: &Q, destpadname: R, filter: S) -> Result<(), glib::error::BoolError> {
        let srcpadname = srcpadname.into();
        let srcpadname = srcpadname.to_glib_none();
        let destpadname = destpadname.into();
        let destpadname = destpadname.to_glib_none();
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_link_pads_filtered(self.to_glib_none().0, srcpadname.0, dest.to_glib_none().0, destpadname.0, filter.0), "Failed to link pads")
        }
    }

    //fn link_pads_full<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Element>, R: Into<Option<&'b str>>>(&self, srcpadname: P, dest: &Q, destpadname: R, flags: /*Ignored*/PadLinkCheck) -> bool {
    //    unsafe { TODO: call ffi::gst_element_link_pads_full() }
    //}

    fn lost_state(&self) {
        unsafe {
            ffi::gst_element_lost_state(self.to_glib_none().0);
        }
    }

    //fn message_full<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, type_: /*Ignored*/MessageType, domain: /*Ignored*/glib::Quark, code: i32, text: P, debug: Q, file: &str, function: &str, line: i32) {
    //    unsafe { TODO: call ffi::gst_element_message_full() }
    //}

    //#[cfg(feature = "v1_10")]
    //fn message_full_with_details<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, type_: /*Ignored*/MessageType, domain: /*Ignored*/glib::Quark, code: i32, text: P, debug: Q, file: &str, function: &str, line: i32, structure: &mut Structure) {
    //    unsafe { TODO: call ffi::gst_element_message_full_with_details() }
    //}

    fn no_more_pads(&self) {
        unsafe {
            ffi::gst_element_no_more_pads(self.to_glib_none().0);
        }
    }

    fn post_message(&self, message: &Message) -> bool {
        unsafe {
            from_glib(ffi::gst_element_post_message(self.to_glib_none().0, message.to_glib_full()))
        }
    }

    fn provide_clock(&self) -> Option<Clock> {
        unsafe {
            from_glib_full(ffi::gst_element_provide_clock(self.to_glib_none().0))
        }
    }

    fn query_convert(&self, src_format: Format, src_val: i64, dest_format: Format) -> Option<i64> {
        unsafe {
            let mut dest_val = mem::uninitialized();
            let ret = from_glib(ffi::gst_element_query_convert(self.to_glib_none().0, src_format.to_glib(), src_val, dest_format.to_glib(), &mut dest_val));
            if ret { Some(dest_val) } else { None }
        }
    }

    fn query_duration(&self, format: Format) -> Option<i64> {
        unsafe {
            let mut duration = mem::uninitialized();
            let ret = from_glib(ffi::gst_element_query_duration(self.to_glib_none().0, format.to_glib(), &mut duration));
            if ret { Some(duration) } else { None }
        }
    }

    fn query_position(&self, format: Format) -> Option<i64> {
        unsafe {
            let mut cur = mem::uninitialized();
            let ret = from_glib(ffi::gst_element_query_position(self.to_glib_none().0, format.to_glib(), &mut cur));
            if ret { Some(cur) } else { None }
        }
    }

    fn release_request_pad<P: IsA<Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_element_release_request_pad(self.to_glib_none().0, pad.to_glib_none().0);
        }
    }

    fn remove_pad<P: IsA<Pad>>(&self, pad: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_remove_pad(self.to_glib_none().0, pad.to_glib_full()), "Failed to remove pad")
        }
    }

    #[cfg(feature = "v1_10")]
    fn remove_property_notify_watch(&self, watch_id: libc::c_ulong) {
        unsafe {
            ffi::gst_element_remove_property_notify_watch(self.to_glib_none().0, watch_id);
        }
    }

    fn request_pad<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(&self, templ: &PadTemplate, name: P, caps: Q) -> Option<Pad> {
        let name = name.into();
        let name = name.to_glib_none();
        let caps = caps.into();
        let caps = caps.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_element_request_pad(self.to_glib_none().0, templ.to_glib_none().0, name.0, caps.0))
        }
    }

    fn seek(&self, rate: f64, format: Format, flags: SeekFlags, start_type: SeekType, start: i64, stop_type: SeekType, stop: i64) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_seek(self.to_glib_none().0, rate, format.to_glib(), flags.to_glib(), start_type.to_glib(), start, stop_type.to_glib(), stop), "Failed to seek")
        }
    }

    fn seek_simple(&self, format: Format, seek_flags: SeekFlags, seek_pos: i64) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_seek_simple(self.to_glib_none().0, format.to_glib(), seek_flags.to_glib(), seek_pos), "Failed to seek")
        }
    }

    fn send_event(&self, event: &Event) -> bool {
        unsafe {
            from_glib(ffi::gst_element_send_event(self.to_glib_none().0, event.to_glib_full()))
        }
    }

    fn set_base_time(&self, time: ClockTime) {
        unsafe {
            ffi::gst_element_set_base_time(self.to_glib_none().0, time);
        }
    }

    fn set_bus(&self, bus: &Bus) {
        unsafe {
            ffi::gst_element_set_bus(self.to_glib_none().0, bus.to_glib_none().0);
        }
    }

    fn set_clock<P: IsA<Clock>>(&self, clock: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_set_clock(self.to_glib_none().0, clock.to_glib_none().0), "Failed to set clock")
        }
    }

    //fn set_context(&self, context: /*Ignored*/&mut Context) {
    //    unsafe { TODO: call ffi::gst_element_set_context() }
    //}

    fn set_locked_state(&self, locked_state: bool) -> bool {
        unsafe {
            from_glib(ffi::gst_element_set_locked_state(self.to_glib_none().0, locked_state.to_glib()))
        }
    }

    fn set_start_time(&self, time: ClockTime) {
        unsafe {
            ffi::gst_element_set_start_time(self.to_glib_none().0, time);
        }
    }

    fn set_state(&self, state: State) -> StateChangeReturn {
        unsafe {
            from_glib(ffi::gst_element_set_state(self.to_glib_none().0, state.to_glib()))
        }
    }

    fn sync_state_with_parent(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_element_sync_state_with_parent(self.to_glib_none().0), "Failed to sync state with parent")
        }
    }

    fn unlink<P: IsA<Element>>(&self, dest: &P) {
        unsafe {
            ffi::gst_element_unlink(self.to_glib_none().0, dest.to_glib_none().0);
        }
    }

    //fn unlink_many<P: IsA<Element>>(&self, element_2: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_element_unlink_many() }
    //}

    fn unlink_pads<P: IsA<Element>>(&self, srcpadname: &str, dest: &P, destpadname: &str) {
        unsafe {
            ffi::gst_element_unlink_pads(self.to_glib_none().0, srcpadname.to_glib_none().0, dest.to_glib_none().0, destpadname.to_glib_none().0);
        }
    }

    fn connect_no_more_pads<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "no-more-pads",
                transmute(no_more_pads_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_pad_added<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Pad) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pad-added",
                transmute(pad_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_pad_removed<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Pad) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pad-removed",
                transmute(pad_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn no_more_pads_trampoline<P>(this: *mut ffi::GstElement, f: glib_ffi::gpointer)
where P: IsA<Element> {
    callback_guard!();
    let f: &Box_<Fn(&P) + Send + Sync + 'static> = transmute(f);
    f(&Element::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn pad_added_trampoline<P>(this: *mut ffi::GstElement, new_pad: *mut ffi::GstPad, f: glib_ffi::gpointer)
where P: IsA<Element> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Pad) + Send + Sync + 'static> = transmute(f);
    f(&Element::from_glib_none(this).downcast_unchecked(), &from_glib_none(new_pad))
}

unsafe extern "C" fn pad_removed_trampoline<P>(this: *mut ffi::GstElement, old_pad: *mut ffi::GstPad, f: glib_ffi::gpointer)
where P: IsA<Element> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Pad) + Send + Sync + 'static> = transmute(f);
    f(&Element::from_glib_none(this).downcast_unchecked(), &from_glib_none(old_pad))
}
