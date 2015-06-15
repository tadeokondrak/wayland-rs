use libc::{c_int, c_char, c_void, int32_t, uint32_t, size_t};

#[repr(C)] pub struct wl_argument;

#[repr(C)]
pub struct wl_array {
    pub size: size_t,
    pub alloc: size_t,
    pub data: *mut c_void
}

#[repr(C)] pub struct wl_display;
#[repr(C)] pub struct wl_event_queue;

/// Type representing an interface in the `libwayland-client.so` ABI.
///
/// This type allows you to manually bind to a wayland protocol extension
/// not (yet?) supported by this library, via the `FFI` and `Bind` traits.
#[repr(C)]
pub struct wl_interface {
    pub name: *const char,
    pub version: c_int,
    pub method_count: c_int,
    pub methods: *const wl_message,
    pub event_count: c_int,
    pub events: *const wl_message,
}

#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

/// Type representing a message in the `libwayland-client.so` ABI.
#[repr(C)]
pub struct wl_message {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub types: *const *mut wl_interface,
}

#[repr(C)] pub struct wl_proxy;

#[repr(C)] pub type wl_dispatcher_func_t = extern fn(*const c_void, 
                                                     *mut c_void,
                                                     uint32_t,
                                                     *const wl_message,
                                                     *mut wl_argument
                                                    );
#[repr(C)] pub type wl_log_func_t = extern fn(_: *const c_char, ...);

#[repr(C)] pub type wl_fixed_t = int32_t;

pub fn wl_fixed_to_double(f: wl_fixed_t) -> f64 {
    f as f64 / 256.
}

pub fn wl_fixed_from_double(d: f64) -> wl_fixed_t {
    (d * 256.) as i32
}

#[cfg(feature = "dlopen")]
external_library!(WaylandClient,
    // interfaces
    wl_buffer_interface: &'static wl_interface,
    wl_callback_interface: &'static wl_interface,
    wl_compositor_interface: &'static wl_interface,
    wl_data_device_interface: &'static wl_interface,
    wl_data_device_manager_interface: &'static wl_interface,
    wl_data_offer_interface: &'static wl_interface,
    wl_data_source_interface: &'static wl_interface,
    wl_display_interface: &'static wl_interface,
    wl_keyboard_interface: &'static wl_interface,
    wl_output_interface: &'static wl_interface,
    wl_pointer_interface: &'static wl_interface,
    wl_region_interface: &'static wl_interface,
    wl_registry_interface: &'static wl_interface,
    wl_seat_interface: &'static wl_interface,
    wl_shell_interface: &'static wl_interface,
    wl_shell_surface_interface: &'static wl_interface,
    wl_shm_interface: &'static wl_interface,
    wl_shm_pool_interface: &'static wl_interface,
    wl_subcompositor_interface: &'static wl_interface,
    wl_subsurface_interface: &'static wl_interface,
    wl_surface_interface: &'static wl_interface,
    wl_touch_interface: &'static wl_interface,

    // display creation and destruction
    wl_display_connect_to_fd: unsafe extern fn(fd: c_int) -> *mut wl_display,
    wl_display_connect: unsafe extern fn(name: *const c_char) -> *mut wl_display,
    wl_display_disconnect: unsafe extern fn(display: *mut wl_display),
    wl_display_get_fd: unsafe extern fn(display: *mut wl_display) -> c_int,
    // display events handling
    wl_display_roundtrip: unsafe extern fn(display: *mut wl_display) -> c_int,
    wl_display_read_events: unsafe extern fn(display: *mut wl_display) -> c_int,
    wl_display_prepare_read: unsafe extern fn(display: *mut wl_display) -> c_int,
    wl_display_cancel_read: unsafe extern fn(display: *mut wl_display),
    wl_display_dispatch: unsafe extern fn(display: *mut wl_display) -> c_int,
    wl_display_dispatch_pending: unsafe extern fn(display: *mut wl_display) -> c_int,
    // error handling
    wl_display_get_error: unsafe extern fn(display: *mut wl_display) -> c_int,
    wl_display_get_protocol_error: unsafe extern fn(display: *mut wl_display,
                                                    interface: *mut *mut wl_interface,
                                                    id: *mut uint32_t
                                                   ) -> uint32_t,
    // requests handling
    wl_display_flush: unsafe extern fn(display: *mut wl_display) -> c_int,

    // event queues
    wl_event_queue_destroy: unsafe extern fn(queue: *mut wl_event_queue),
    wl_display_create_queue: unsafe extern fn(display: *mut wl_display) -> *mut wl_event_queue,
    wl_display_roundtrip_queue: unsafe extern fn(display: *mut wl_display,
                                                 queue: *mut wl_event_queue
                                                ) -> c_int,
    wl_display_prepare_read_queue: unsafe extern fn(display: *mut wl_display,
                                                    queue: *mut wl_event_queue
                                                   ) -> c_int,
    wl_display_dispatch_queue: unsafe extern fn(display: *mut wl_display,
                                                queue: *mut wl_event_queue
                                               ) -> c_int,
    wl_display_dispatch_queue_pending: unsafe extern fn(display: *mut wl_display,
                                                        queue: *mut wl_event_queue
                                                       ) -> c_int,

    // proxys
    wl_proxy_create: unsafe extern fn(factory: *mut wl_proxy,
                                      interface: *const wl_interface
                                     ) -> *mut wl_proxy,
    wl_proxy_destroy: unsafe extern fn(proxy: *mut wl_proxy),
    wl_proxy_add_listener: unsafe extern fn(proxy: *mut wl_proxy,
                                            implementation: *mut extern fn(),
                                            data: *mut c_void
                                           ) -> c_int,
    wl_proxy_get_listener: unsafe extern fn(proxy: *mut wl_proxy) -> *const c_void,
    wl_proxy_add_dispatcher: unsafe extern fn(proxy: *mut wl_proxy,
                                              dispatcher: wl_dispatcher_func_t,
                                              implementation: *const c_void,
                                              data: *mut c_void
                                             ) -> c_int,
    wl_proxy_marshal_array_constructor: unsafe extern fn(proxy: *mut wl_proxy,
                                                         opcode: uint32_t,
                                                         args: *mut wl_argument,
                                                         interface: *const wl_interface
                                                        ) -> c_int,
    wl_proxy_marshal: unsafe extern fn(proxy: *mut wl_proxy,
                                       opcode: uint32_t,
                                       ...
                                      ),
    wl_proxy_marshal_constructor: unsafe extern fn(proxy: *mut wl_proxy,
                                                   opcode: uint32_t,
                                                   interface: *const wl_interface,
                                                   ...
                                                  ) -> *mut wl_proxy,
    wl_proxy_marshal_array: unsafe extern fn(proxy: *mut wl_proxy,
                                             opcode: uint32_t,
                                             args: *mut wl_argument
                                            ),
    wl_proxy_set_user_data: unsafe extern fn(proxy: *mut wl_proxy,
                                             data: *mut c_void
                                            ),
    wl_proxy_get_user_data: unsafe extern fn(proxy: *mut wl_proxy) -> *mut c_void,
    wl_proxy_get_id: unsafe extern fn(proxy: *mut wl_proxy) -> uint32_t,
    wl_proxy_get_class: unsafe extern fn(proxy: *mut wl_proxy) -> *const c_char,
    wl_proxy_set_queue: unsafe extern fn(proxy: *mut wl_proxy,
                                         queue: *mut wl_event_queue
                                        ),

    // log
    wl_log_set_handler_client: unsafe extern fn(handler: wl_log_func_t),
    // wl_log: unsafe extern fn(fmt: *const c_char, ...),

    // lists
    wl_list_init: unsafe extern fn(list: *mut wl_list),
    wl_list_insert: unsafe extern fn(list: *mut wl_list, elm: *mut wl_list),
    wl_list_remove: unsafe extern fn(elm: *mut wl_list),
    wl_list_length: unsafe extern fn(list: *const wl_list) -> c_int,
    wl_list_empty: unsafe extern fn(list: *const wl_list) -> c_int,
    wl_list_insert_list: unsafe extern fn(list: *mut wl_list, other: *mut wl_list),

    // arrays
    wl_array_init: unsafe extern fn(array: *mut wl_array),
    wl_array_release: unsafe extern fn(array: *mut wl_array),
    wl_array_add: unsafe extern fn(array: *mut wl_array, size: size_t),
    wl_array_copy: unsafe extern fn(array: *mut wl_array, source: *mut wl_array)
);

#[cfg(feature = "dlopen")]
lazy_static!(
    pub static ref WAYLAND_CLIENT_OPTION: Option<WaylandClient> = { 
        WaylandClient::open("libwayland-client.so")
    };
    pub static ref WAYLAND_CLIENT_HANDLE: &'static WaylandClient = {
        WAYLAND_CLIENT_OPTION.as_ref().expect("Library libwayland-client.so could not be loaded.")
    };
);

#[cfg(not(feature = "dlopen"))]
#[link(name = "wayland-client")]
extern {
    pub static wl_buffer_interface: wl_interface;
    pub static wl_callback_interface: wl_interface;
    pub static wl_compositor_interface: wl_interface;
    pub static wl_data_device_interface: wl_interface;
    pub static wl_data_device_manager_interface: wl_interface;
    pub static wl_data_offer_interface: wl_interface;
    pub static wl_data_source_interface: wl_interface;
    pub static wl_display_interface: wl_interface;
    pub static wl_keyboard_interface: wl_interface;
    pub static wl_output_interface: wl_interface;
    pub static wl_pointer_interface: wl_interface;
    pub static wl_region_interface: wl_interface;
    pub static wl_registry_interface: wl_interface;
    pub static wl_seat_interface: wl_interface;
    pub static wl_shell_interface: wl_interface;
    pub static wl_shell_surface_interface: wl_interface;
    pub static wl_shm_interface: wl_interface;
    pub static wl_shm_pool_interface: wl_interface;
    pub static wl_subcompositor_interface: wl_interface;
    pub static wl_subsurface_interface: wl_interface;
    pub static wl_surface_interface: wl_interface;
    pub static wl_touch_interface: wl_interface;

    // display creation and destruction
    pub fn wl_display_connect_to_fd(fd: c_int) -> *mut wl_display;
    pub fn wl_display_connect(name: *const c_char) -> *mut wl_display;
    pub fn wl_display_disconnect(display: *mut wl_display);
    pub fn wl_display_get_fd(display: *mut wl_display) -> c_int;
    // display events handling
    pub fn wl_display_roundtrip(display: *mut wl_display) -> c_int;
    pub fn wl_display_read_events(display: *mut wl_display) -> c_int;
    pub fn wl_display_prepare_read(display: *mut wl_display) -> c_int;
    pub fn wl_display_cancel_read(display: *mut wl_display);
    pub fn wl_display_dispatch(display: *mut wl_display) -> c_int;
    pub fn wl_display_dispatch_pending(display: *mut wl_display) -> c_int;
    // error handling
    pub fn wl_display_get_error(display: *mut wl_display) -> c_int;
    pub fn wl_display_get_protocol_error(display: *mut wl_display,
                                                    interface: *mut *mut wl_interface,
                                                    id: *mut uint32_t
                                                   ) -> uint32_t;
    // requests handling
    pub fn wl_display_flush(display: *mut wl_display) -> c_int;

    // event queues
    pub fn wl_event_queue_destroy(queue: *mut wl_event_queue);
    pub fn wl_display_create_queue(display: *mut wl_display) -> *mut wl_event_queue;
    pub fn wl_display_roundtrip_queue(display: *mut wl_display,
                                                 queue: *mut wl_event_queue
                                                ) -> c_int;
    pub fn wl_display_prepare_read_queue(display: *mut wl_display,
                                                    queue: *mut wl_event_queue
                                                   ) -> c_int;
    pub fn wl_display_dispatch_queue(display: *mut wl_display,
                                                queue: *mut wl_event_queue
                                               ) -> c_int;
    pub fn wl_display_dispatch_queue_pending(display: *mut wl_display,
                                                        queue: *mut wl_event_queue
                                                       ) -> c_int;

    // proxys
    pub fn wl_proxy_create(factory: *mut wl_proxy,
                                      interface: *const wl_interface
                                     ) -> *mut wl_proxy;
    pub fn wl_proxy_destroy(proxy: *mut wl_proxy);
    pub fn wl_proxy_add_listener(proxy: *mut wl_proxy,
                                            implementation: *mut extern fn(),
                                            data: *mut c_void
                                           ) -> c_int;
    pub fn wl_proxy_get_listener(proxy: *mut wl_proxy) -> *const c_void;
    pub fn wl_proxy_add_dispatcher(proxy: *mut wl_proxy,
                                              dispatcher: wl_dispatcher_func_t,
                                              implementation: *const c_void,
                                              data: *mut c_void
                                             ) -> c_int;
    pub fn wl_proxy_marshal_array_constructor(proxy: *mut wl_proxy,
                                                         opcode: uint32_t,
                                                         args: *mut wl_argument,
                                                         interface: *const wl_interface
                                                        ) -> c_int;
    pub fn wl_proxy_marshal(proxy: *mut wl_proxy,
                                       opcode: uint32_t,
                                       ...
                                      );
    pub fn wl_proxy_marshal_constructor(proxy: *mut wl_proxy,
                                                   opcode: uint32_t,
                                                   interface: *const wl_interface,
                                                   ...
                                                  ) -> *mut wl_proxy;
    pub fn wl_proxy_marshal_array(proxy: *mut wl_proxy,
                                             opcode: uint32_t,
                                             args: *mut wl_argument
                                            );
    pub fn wl_proxy_set_user_data(proxy: *mut wl_proxy,
                                             data: *mut c_void
                                            );
    pub fn wl_proxy_get_user_data(proxy: *mut wl_proxy) -> *mut c_void;
    pub fn wl_proxy_get_id(proxy: *mut wl_proxy) -> uint32_t;
    pub fn wl_proxy_get_class(proxy: *mut wl_proxy) -> *const c_char;
    pub fn wl_proxy_set_queue(proxy: *mut wl_proxy,
                                         queue: *mut wl_event_queue
                                        );

    // log
    pub fn wl_log_set_handler_client(handler: wl_log_func_t);
    // wl_log(fmt: *const c_char, ...);

    // lists
    pub fn wl_list_init(list: *mut wl_list);
    pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    pub fn wl_list_remove(elm: *mut wl_list);
    pub fn wl_list_length(list: *const wl_list) -> c_int;
    pub fn wl_list_empty(list: *const wl_list) -> c_int;
    pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);

    // arrays
    pub fn wl_array_init(array: *mut wl_array);
    pub fn wl_array_release(array: *mut wl_array);
    pub fn wl_array_add(array: *mut wl_array, size: size_t);
    pub fn wl_array_copy(array: *mut wl_array, source: *mut wl_array);
}