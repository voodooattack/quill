use bevy::prelude::*;

use crate::{Cx, TrackingScope};

pub(crate) trait CallbackFnRef<P> {
    fn call(&self, cx: &mut Cx, props: P);
}

impl<P, F: Fn(&mut Cx, P)> CallbackFnRef<P> for F {
    fn call(&self, cx: &mut Cx, props: P) {
        self(cx, props);
    }
}

pub(crate) trait CallbackFnMutRef<P> {
    fn call(&mut self, cx: &mut Cx, props: P);
}

impl<P, F: FnMut(&mut Cx, P)> CallbackFnMutRef<P> for F {
    fn call(&mut self, cx: &mut Cx, props: P) {
        self(cx, props);
    }
}

/// Contains a boxed, type-erased callback.
#[derive(Component)]
pub(crate) struct CallbackFnCell<P> {
    pub(crate) inner: Option<Box<dyn CallbackFnRef<P> + Send + Sync>>,
}

#[derive(Component)]
pub(crate) struct CallbackFnMutCell<P> {
    pub(crate) inner: Option<Box<dyn CallbackFnMutRef<P> + Send + Sync>>,
}

/// Contains a reference to a callback. `P` is the type of the props.
#[derive(PartialEq)]
pub struct Callback<P = ()> {
    pub(crate) id: Entity,
    pub(crate) marker: std::marker::PhantomData<P>,
}

impl<P> Copy for Callback<P> {}
impl<P> Clone for Callback<P> {
    fn clone(&self) -> Self {
        *self
    }
}

pub trait RunCallback {
    fn run_callback<P: 'static>(&mut self, callback: Callback<P>, props: P);
}

/// A mutable reactive context. This allows write access to reactive data sources.
impl RunCallback for World {
    /// Invoke a callback with the given props.
    ///
    /// Arguments:
    /// * `callback` - The callback to invoke.
    /// * `props` - The props to pass to the callback.
    fn run_callback<P: 'static>(&mut self, callback: Callback<P>, props: P) {
        let tick = self.change_tick();
        let mut tracking = TrackingScope::new(tick);
        let mut callback_entity = self.entity_mut(callback.id);
        if let Some(mut cell) = callback_entity.get_mut::<CallbackFnCell<P>>() {
            let mut callback_fn = cell.inner.take();
            let callback_box = callback_fn.as_ref().expect("Callback is not present");
            let mut cx = Cx::new(self, callback.id, &mut tracking);
            callback_box.call(&mut cx, props);
            let mut callback_entity = self.entity_mut(callback.id);
            callback_entity
                .get_mut::<CallbackFnCell<P>>()
                .unwrap()
                .inner = callback_fn.take();
        } else if let Some(mut cell) = callback_entity.get_mut::<CallbackFnMutCell<P>>() {
            let mut callback_fn = cell.inner.take();
            let callback_box = callback_fn.as_mut().expect("Callback is not present");
            let mut cx = Cx::new(self, callback.id, &mut tracking);
            callback_box.call(&mut cx, props);
            let mut callback_entity = self.entity_mut(callback.id);
            callback_entity
                .get_mut::<CallbackFnMutCell<P>>()
                .unwrap()
                .inner = callback_fn.take();
        } else {
            warn!("No callback found for {:?}", callback.id);
        }
    }
}

impl<'p, 'w> RunCallback for Cx<'p, 'w> {
    fn run_callback<P: 'static>(&mut self, callback: Callback<P>, props: P) {
        self.world_mut().run_callback(callback, props);
    }
}