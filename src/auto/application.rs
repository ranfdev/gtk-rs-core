// This file was generated by gir (ce03df6) from gir-files (71d73f0)
// DO NOT EDIT

use ApplicationInhibitFlags;
use Window;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Application(Object<ffi::GtkApplication>): [
        gio::Application => gio_ffi::GApplication,
        gio::ActionGroup => gio_ffi::GActionGroup,
        gio::ActionMap => gio_ffi::GActionMap,
    ];

    match fn {
        get_type => || ffi::gtk_application_get_type(),
    }
}

pub trait ApplicationExt {
    fn add_accelerator<'a, P: Into<Option<&'a glib::Variant>>>(&self, accelerator: &str, action_name: &str, parameter: P);

    fn add_window<P: IsA<Window>>(&self, window: &P);

    #[cfg(feature = "v3_12")]
    fn get_accels_for_action(&self, detailed_action_name: &str) -> Vec<String>;

    #[cfg(feature = "v3_14")]
    fn get_actions_for_accel(&self, accel: &str) -> Vec<String>;

    #[cfg(feature = "v3_6")]
    fn get_active_window(&self) -> Option<Window>;

    fn get_app_menu(&self) -> Option<gio::MenuModel>;

    #[cfg(feature = "v3_14")]
    fn get_menu_by_id(&self, id: &str) -> Option<gio::Menu>;

    fn get_menubar(&self) -> Option<gio::MenuModel>;

    #[cfg(feature = "v3_6")]
    fn get_window_by_id(&self, id: u32) -> Option<Window>;

    fn get_windows(&self) -> Vec<Window>;

    fn inhibit<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, window: Q, flags: ApplicationInhibitFlags, reason: R) -> u32;

    fn is_inhibited(&self, flags: ApplicationInhibitFlags) -> bool;

    #[cfg(feature = "v3_12")]
    fn list_action_descriptions(&self) -> Vec<String>;

    #[cfg(feature = "v3_14")]
    fn prefers_app_menu(&self) -> bool;

    fn remove_accelerator<'a, P: Into<Option<&'a glib::Variant>>>(&self, action_name: &str, parameter: P);

    fn remove_window<P: IsA<Window>>(&self, window: &P);

    #[cfg(feature = "v3_12")]
    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]);

    fn set_app_menu<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, app_menu: Q);

    fn set_menubar<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, menubar: Q);

    fn uninhibit(&self, cookie: u32);

    fn get_property_active_window(&self) -> Option<Window>;

    fn get_property_register_session(&self) -> bool;

    fn set_property_register_session(&self, register_session: bool);

    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> u64;

    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Application> + IsA<glib::object::Object>> ApplicationExt for O {
    fn add_accelerator<'a, P: Into<Option<&'a glib::Variant>>>(&self, accelerator: &str, action_name: &str, parameter: P) {
        let parameter = parameter.into();
        let parameter = parameter.to_glib_none();
        unsafe {
            ffi::gtk_application_add_accelerator(self.to_glib_none().0, accelerator.to_glib_none().0, action_name.to_glib_none().0, parameter.0);
        }
    }

    fn add_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_application_add_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_accels_for_action(&self, detailed_action_name: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_accels_for_action(self.to_glib_none().0, detailed_action_name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_actions_for_accel(&self, accel: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_actions_for_accel(self.to_glib_none().0, accel.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_active_window(self.to_glib_none().0))
        }
    }

    fn get_app_menu(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_app_menu(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_menu_by_id(&self, id: &str) -> Option<gio::Menu> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menu_by_id(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    fn get_menubar(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menubar(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_window_by_id(&self, id: u32) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_window_by_id(self.to_glib_none().0, id))
        }
    }

    fn get_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_application_get_windows(self.to_glib_none().0))
        }
    }

    fn inhibit<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, window: Q, flags: ApplicationInhibitFlags, reason: R) -> u32 {
        let window = window.into();
        let window = window.to_glib_none();
        let reason = reason.into();
        let reason = reason.to_glib_none();
        unsafe {
            ffi::gtk_application_inhibit(self.to_glib_none().0, window.0, flags.to_glib(), reason.0)
        }
    }

    fn is_inhibited(&self, flags: ApplicationInhibitFlags) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_is_inhibited(self.to_glib_none().0, flags.to_glib()))
        }
    }

    #[cfg(feature = "v3_12")]
    fn list_action_descriptions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_list_action_descriptions(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn prefers_app_menu(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_prefers_app_menu(self.to_glib_none().0))
        }
    }

    fn remove_accelerator<'a, P: Into<Option<&'a glib::Variant>>>(&self, action_name: &str, parameter: P) {
        let parameter = parameter.into();
        let parameter = parameter.to_glib_none();
        unsafe {
            ffi::gtk_application_remove_accelerator(self.to_glib_none().0, action_name.to_glib_none().0, parameter.0);
        }
    }

    fn remove_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_application_remove_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]) {
        unsafe {
            ffi::gtk_application_set_accels_for_action(self.to_glib_none().0, detailed_action_name.to_glib_none().0, accels.to_glib_none().0);
        }
    }

    fn set_app_menu<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, app_menu: Q) {
        let app_menu = app_menu.into();
        let app_menu = app_menu.to_glib_none();
        unsafe {
            ffi::gtk_application_set_app_menu(self.to_glib_none().0, app_menu.0);
        }
    }

    fn set_menubar<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, menubar: Q) {
        let menubar = menubar.into();
        let menubar = menubar.to_glib_none();
        unsafe {
            ffi::gtk_application_set_menubar(self.to_glib_none().0, menubar.0);
        }
    }

    fn uninhibit(&self, cookie: u32) {
        unsafe {
            ffi::gtk_application_uninhibit(self.to_glib_none().0, cookie);
        }
    }

    fn get_property_active_window(&self) -> Option<Window> {
        let mut value = Value::from(None::<&Window>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active-window".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_register_session(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "register-session".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_register_session(&self, register_session: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "register-session".to_glib_none().0, Value::from(&register_session).to_glib_none().0);
        }
    }

    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Window) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-added",
                transmute(window_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Window) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-removed",
                transmute(window_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn window_added_trampoline<P>(this: *mut ffi::GtkApplication, window: *mut ffi::GtkWindow, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Window) + 'static> = transmute(f);
    f(&Application::from_glib_none(this).downcast_unchecked(), &from_glib_none(window))
}

unsafe extern "C" fn window_removed_trampoline<P>(this: *mut ffi::GtkApplication, window: *mut ffi::GtkWindow, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Window) + 'static> = transmute(f);
    f(&Application::from_glib_none(this).downcast_unchecked(), &from_glib_none(window))
}
