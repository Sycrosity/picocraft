use core::borrow::Borrow;

use crate::prelude::*;

impl<T: Clone> Clone for PicoCow<'_, T> {
    fn clone(&self) -> Self {
        match *self {
            Self::Borrowed(t) => Self::Borrowed(t),
            Self::Owned(ref o) => Self::Owned(o.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (&mut Self::Owned(ref mut dest), Self::Owned(o)) => dest.clone_from(o),
            (t, s) => *t = s.clone(),
        }
    }
}

impl<T: Clone> PicoCow<'_, T> {
    pub const fn is_borrowed(&self) -> bool {
        match *self {
            Self::Borrowed(_) => true,
            Self::Owned(_) => false,
        }
    }

    pub const fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    pub fn to_mut(&mut self) -> &mut T {
        match *self {
            Self::Borrowed(borrowed) => {
                *self = Self::Owned(borrowed.clone());
                match *self {
                    Self::Borrowed(..) => unreachable!(),
                    Self::Owned(ref mut owned) => owned,
                }
            }
            Self::Owned(ref mut owned) => owned,
        }
    }

    pub fn into_owned(self) -> T {
        match self {
            Self::Borrowed(borrowed) => borrowed.clone(),
            Self::Owned(owned) => owned,
        }
    }
}

impl<T: Clone> core::ops::Deref for PicoCow<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match *self {
            Self::Borrowed(borrowed) => borrowed,
            Self::Owned(ref owned) => owned.borrow(),
        }
    }
}
