use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub struct IgnoreDerive<T>(T);

impl<T> IgnoreDerive<T> {
    pub fn take(self) -> T {
        self.0
    }
}

impl<T> Debug for IgnoreDerive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**Ignored**")
    }
}

impl<T> PartialEq for IgnoreDerive<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T: Clone> Clone for IgnoreDerive<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for IgnoreDerive<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for IgnoreDerive<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for IgnoreDerive<T> {
    fn from(value: T) -> Self {
        IgnoreDerive(value)
    }
}
