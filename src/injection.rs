//! Implementation details of injections - values that come from running effects.

use core::marker::PhantomData;

/// Before an effectful computation has started, there is no injection to pass in, because no
/// effects have been run yet. However, due to the signature of
/// [`Generator::resume`](core::ops::Generator::resume), it is necessary to pass one in anyway.
/// This type is used as a first injection for all effectful computations.
pub struct Begin;

/// Tagging a value with `PhantomData` of another type allows it to be distinguished from other
/// occurrences of the same type in a coproduct. If two effects' injections were both `i32`, it
/// would be impossible to tell the injections apart without tagging them with the effect that they
/// come from.
pub struct Tagged<T, Tag>(T, PhantomData<Tag>);

impl<T, Tag> Tagged<T, Tag> {
    pub fn new(v: T) -> Self {
        Tagged(v, PhantomData)
    }

    pub fn untag(self) -> T {
        self.0
    }
}
