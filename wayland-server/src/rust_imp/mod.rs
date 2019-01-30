use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use downcast::Downcast;

use wayland_commons::map::ObjectMap;
use wayland_commons::wire::Message;
use wayland_commons::MessageGroup;

use {Interface, NewResource, Resource};

mod clients;
mod display;
mod event_loop_glue;
mod globals;
mod resources;

pub(crate) use self::clients::ClientInner;
pub(crate) use self::display::DisplayInner;
pub(crate) use self::globals::GlobalInner;
pub(crate) use self::resources::{NewResourceInner, ResourceInner};

/// A handle to the object map internal to the lib state
///
/// This type is only usable by code generated by `wayland-scanner`, and is
/// not instantiable directly.
pub struct ResourceMap {
    map: Arc<Mutex<ObjectMap<self::resources::ObjectMeta>>>,
    client: ClientInner,
}

impl ResourceMap {
    fn make(map: Arc<Mutex<ObjectMap<self::resources::ObjectMeta>>>, client: ClientInner) -> ResourceMap {
        ResourceMap { map, client }
    }

    /// Retrieve the Resource corresponding to a given id
    pub fn get<I: Interface>(&mut self, id: u32) -> Option<Resource<I>> {
        ResourceInner::from_id(id, self.map.clone(), self.client.clone()).map(|object| {
            debug_assert!(I::NAME == "<anonymous>" || object.is_interface::<I>());
            Resource::wrap(object)
        })
    }

    /// Create a new resource for a given id
    pub fn get_new<I: Interface>(&mut self, id: u32) -> Option<NewResource<I>> {
        debug_assert!(self
            .map
            .lock()
            .unwrap()
            .find(id)
            .map(|obj| obj.is_interface::<I>())
            .unwrap_or(true));
        NewResourceInner::from_id(id, self.map.clone(), self.client.clone()).map(NewResource::wrap)
    }
}

pub(crate) trait Dispatcher: Downcast + Send {
    fn dispatch(&mut self, msg: Message, resource: ResourceInner, map: &mut ResourceMap) -> Result<(), ()>;
    fn destroy(&mut self, resource: ResourceInner);
}

mod dispatcher_impl {
    // this mod has for sole purpose to allow to silence these `dead_code` warnings...
    #![allow(dead_code)]
    use super::Dispatcher;
    impl_downcast!(Dispatcher);
}

pub(crate) struct ImplDispatcher<I: Interface + From<Resource<I>>, F: FnMut(I::Request, I)> {
    _i: ::std::marker::PhantomData<&'static I>,
    implementation: Option<F>,
    destructor: Option<Box<FnMut(I)>>,
}

// This unsafe impl is "technically wrong", but enforced by the fact that
// the Impl will only ever be called from the same EventLoop, which is stuck
// on a single thread. The NewProxy::implement/implement_nonsend methods
// take care of ensuring that any non-Send impl is on the correct thread.
unsafe impl<I, F> Send for ImplDispatcher<I, F>
where
    I: Interface + From<Resource<I>>,
    F: FnMut(I::Request, I) + 'static,
    I::Request: MessageGroup<Map = ResourceMap>,
{
}

impl<I, F> Dispatcher for ImplDispatcher<I, F>
where
    I: Interface + From<Resource<I>>,
    F: FnMut(I::Request, I) + 'static,
    I::Request: MessageGroup<Map = ResourceMap>,
{
    fn dispatch(&mut self, msg: Message, resource: ResourceInner, map: &mut ResourceMap) -> Result<(), ()> {
        if ::std::env::var_os("WAYLAND_DEBUG").is_some() {
            eprintln!(
                " <- {}@{}: {} {:?}",
                resource.object.interface,
                resource.id,
                resource.object.requests[msg.opcode as usize].name,
                msg.args
            );
        }
        let message = I::Request::from_raw(msg, map)?;
        if message.is_destructor() {
            resource.object.meta.alive.store(false, Ordering::Release);
            let mut kill = false;
            if let Some(ref mut data) = *resource.client.data.lock().unwrap() {
                data.schedule_destructor(resource.clone());
                kill = data.delete_id(resource.id).is_err();
            }
            if kill {
                resource.client.kill();
            }
            self.implementation.as_mut().unwrap()(message, Resource::<I>::wrap(resource.clone()).into());
        } else {
            self.implementation.as_mut().unwrap()(message, Resource::<I>::wrap(resource).into());
        }
        Ok(())
    }

    fn destroy(&mut self, resource: ResourceInner) {
        self.implementation.take();
        if let Some(mut dest) = self.destructor.take() {
            dest(Resource::<I>::wrap(resource).into())
        }
    }
}

pub(crate) unsafe fn make_dispatcher<I, F, Dest>(
    implementation: F,
    destructor: Option<Dest>,
) -> Arc<Mutex<Dispatcher + Send>>
where
    I: Interface + From<Resource<I>>,
    F: FnMut(I::Request, I) + 'static,
    I::Request: MessageGroup<Map = ResourceMap>,
    Dest: FnMut(I) + 'static,
{
    Arc::new(Mutex::new(ImplDispatcher {
        _i: ::std::marker::PhantomData,
        implementation: Some(implementation),
        destructor: destructor.map(|d| Box::new(d) as Box<_>),
    }))
}

pub(crate) fn default_dispatcher() -> Arc<Mutex<Dispatcher + Send>> {
    struct DefaultDisp;
    impl Dispatcher for DefaultDisp {
        fn dispatch(
            &mut self,
            _msg: Message,
            resource: ResourceInner,
            _map: &mut ResourceMap,
        ) -> Result<(), ()> {
            eprintln!(
                "[wayland-client] Received an event for unimplemented object {}@{}.",
                resource.object.interface, resource.id
            );
            Err(())
        }
        fn destroy(&mut self, _resource: ResourceInner) {}
    }

    Arc::new(Mutex::new(DefaultDisp))
}
