// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Caps;
use Object;
use Pad;
use PadDirection;
use PadPresence;
use ffi;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
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
    pub struct PadTemplate(Object<ffi::GstPadTemplate, ffi::GstPadTemplateClass>): Object;

    match fn {
        get_type => || ffi::gst_pad_template_get_type(),
    }
}

impl PadTemplate {
    pub fn new(name_template: &str, direction: PadDirection, presence: PadPresence, caps: &Caps) -> PadTemplate {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_pad_template_new(name_template.to_glib_none().0, direction.to_glib(), presence.to_glib(), caps.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new_with_gtype(name_template: &str, direction: PadDirection, presence: PadPresence, caps: &Caps, pad_type: glib::types::Type) -> PadTemplate {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_pad_template_new_with_gtype(name_template.to_glib_none().0, direction.to_glib(), presence.to_glib(), caps.to_glib_none().0, pad_type.to_glib()))
        }
    }

    pub fn get_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_template_get_caps(self.to_glib_none().0))
        }
    }

    pub fn pad_created<P: IsA<Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_pad_template_pad_created(self.to_glib_none().0, pad.to_glib_none().0);
        }
    }

    pub fn get_property_direction(&self) -> PadDirection {
        unsafe {
            let mut value = Value::from_type(<PadDirection as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "direction".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_property_gtype(&self) -> glib::types::Type {
        unsafe {
            let mut value = Value::from_type(<glib::types::Type as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "gtype".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_name_template(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "name-template".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn get_property_presence(&self) -> PadPresence {
        unsafe {
            let mut value = Value::from_type(<PadPresence as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "presence".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn connect_pad_created<F: Fn(&PadTemplate, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PadTemplate, &Pad) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pad-created",
                transmute(pad_created_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_caps_notify<F: Fn(&PadTemplate) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PadTemplate) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::caps",
                transmute(notify_caps_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_direction_notify<F: Fn(&PadTemplate) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PadTemplate) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::direction",
                transmute(notify_direction_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn connect_property_gtype_notify<F: Fn(&PadTemplate) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PadTemplate) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::gtype",
                transmute(notify_gtype_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_name_template_notify<F: Fn(&PadTemplate) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PadTemplate) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name-template",
                transmute(notify_name_template_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_presence_notify<F: Fn(&PadTemplate) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&PadTemplate) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::presence",
                transmute(notify_presence_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for PadTemplate {}
unsafe impl Sync for PadTemplate {}

unsafe extern "C" fn pad_created_trampoline(this: *mut ffi::GstPadTemplate, pad: *mut ffi::GstPad, f: glib_ffi::gpointer) {
    let f: &&(Fn(&PadTemplate, &Pad) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(pad))
}

unsafe extern "C" fn notify_caps_trampoline(this: *mut ffi::GstPadTemplate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&PadTemplate) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_direction_trampoline(this: *mut ffi::GstPadTemplate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&PadTemplate) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
unsafe extern "C" fn notify_gtype_trampoline(this: *mut ffi::GstPadTemplate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&PadTemplate) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_name_template_trampoline(this: *mut ffi::GstPadTemplate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&PadTemplate) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_presence_trampoline(this: *mut ffi::GstPadTemplate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&PadTemplate) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
