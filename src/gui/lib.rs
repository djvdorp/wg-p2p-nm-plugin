extern crate libc;
extern crate glib;
extern crate gtk;
extern crate gdk;
extern crate glib_sys;
extern crate gtk_sys;
extern crate gio_sys;
extern crate gobject_sys;
extern crate gdbus;
extern crate drunken_bishop;

extern {
#[link(name = "wg-p2p-vpn-editor")]
    pub fn nm_vpn_editor_plugin_factory (err: *const *mut u8) -> *const u8;
}

mod editor;
mod bulletinboard;
mod vpn_settings;
mod ip_settings;

use vpn_settings::NMConnection;
use editor::Editor;

static mut EDITOR: Option<Editor> = None;

#[no_mangle]
pub fn init(editor: *mut u8, conn: *mut u8) {
    unsafe {
        gtk::set_initialized();
        EDITOR = Some(Editor::new(editor, conn));
    }
}

#[no_mangle]
pub fn get_widget(_editor: *mut u8) -> *mut glib::object::GObject {
    unsafe { EDITOR.as_mut().unwrap().get_widget() }
}

/*
static gboolean
update_connection (NMVpnEditor *iface,
                   NMConnection *connection,
                   GError **error)
*/
#[no_mangle]
pub fn update_connection(_editor: *mut u8,
                         conn: *mut NMConnection,
                         err: *mut *mut u8) -> u8
{
    let editor = unsafe { EDITOR.as_mut().unwrap() };
    if editor.update_connection(conn, err) { 1 } else { 0 }
}

#[no_mangle]
pub fn rust_connect(_plugin: *mut i8,
                    _conn:   *mut i8,
                    _error:  *mut *mut i8) -> u8
{
    // just a dummy for a linking error generated by the cc crate
    unreachable!()
}

