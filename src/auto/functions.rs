// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::auto::enums::Edge;
use crate::auto::enums::Layer;
use gdk;
use glib::object::IsA;
use glib::translate::*;
use gtk;
use gtk_layer_shell_sys;

pub fn auto_exclusive_zone_enable<P: IsA<gtk::Window>>(window: &P) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_auto_exclusive_zone_enable(window.as_ref().to_glib_none().0);
    }
}

pub fn init_for_window<P: IsA<gtk::Window>>(window: &P) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_init_for_window(window.as_ref().to_glib_none().0);
    }
}

pub fn set_anchor<P: IsA<gtk::Window>>(window: &P, edge: Edge, anchor_to_edge: bool) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_anchor(
            window.as_ref().to_glib_none().0,
            edge.to_glib(),
            anchor_to_edge.to_glib(),
        );
    }
}

pub fn set_exclusive_zone<P: IsA<gtk::Window>>(window: &P, exclusive_zone: i32) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_exclusive_zone(
            window.as_ref().to_glib_none().0,
            exclusive_zone,
        );
    }
}

pub fn set_keyboard_interactivity<P: IsA<gtk::Window>>(window: &P, interacitvity: bool) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_keyboard_interactivity(
            window.as_ref().to_glib_none().0,
            interacitvity.to_glib(),
        );
    }
}

pub fn set_layer<P: IsA<gtk::Window>>(window: &P, layer: Layer) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_layer(window.as_ref().to_glib_none().0, layer.to_glib());
    }
}

pub fn set_margin<P: IsA<gtk::Window>>(window: &P, edge: Edge, margin_size: i32) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_margin(
            window.as_ref().to_glib_none().0,
            edge.to_glib(),
            margin_size,
        );
    }
}

pub fn set_monitor<P: IsA<gtk::Window>>(window: &P, monitor: &gdk::Monitor) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_monitor(
            window.as_ref().to_glib_none().0,
            monitor.to_glib_none().0,
        );
    }
}

pub fn set_namespace<P: IsA<gtk::Window>>(window: &P, name_space: &str) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_namespace(
            window.as_ref().to_glib_none().0,
            name_space.to_glib_none().0,
        );
    }
}
