use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::marker::PhantomData;
use std::borrow::Borrow;

pub struct SelfMonadOnce<O, V: ?Sized, F> {
    owner: O,
    func: Cell<Option<F>>,
    phantom: PhantomData<*const V>
}

impl<O, V: ?Sized, F: FnOnce(&O) -> &V> SelfMonadOnce<O, V, F> {
    pub fn new(owner: O, func: F) -> Self {
        SelfMonadOnce {
            owner,
            func: Cell::new(Some(func)),
            phantom: PhantomData
        }
    }
}

impl<O, V: ?Sized, F: FnOnce(&mut O) -> &mut V> SelfMonadOnce<O, V, F> {
    pub fn new_mut(owner: O, func: F) -> Self {
        SelfMonadOnce {
            owner,
            func: Cell::new(Some(func)),
            phantom: PhantomData
        }
    }
}

impl<O, V: ?Sized, F: FnOnce(&O) -> &V> Deref for SelfMonadOnce<O, V, F> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        AsRef::as_ref(self)
    }
}

impl<O, V: ?Sized, F: FnOnce(&O) -> &V> AsRef<V> for SelfMonadOnce<O, V, F> {
    fn as_ref(&self) -> &V {
        (self.func.take().unwrap())(&self.owner)
    }
}

impl<O, V: ?Sized, F: FnOnce(&mut O) -> &mut V> AsMut<V> for SelfMonadOnce<O, V, F> {
    fn as_mut(&mut self) -> &mut V {
        (self.func.take().unwrap())(&mut self.owner)
    }
}

///-------------------------------------------------------------------------------------------------

pub struct SelfMonad<O, V: ?Sized, F> {
    owner: O,
    func: F,
    phantom: PhantomData<*const V>
}

impl<O, V: ?Sized, F: Fn(&O) -> &V> SelfMonad<O, V, F> {
    pub fn new(owner: O, func: F) -> Self {
        SelfMonad {
            owner,
            func,
            phantom: PhantomData
        }
    }
}

impl<'a, O: 'a, V: ?Sized, F: Fn(&mut O) -> &mut V> SelfMonad<O, V, F> {
    pub fn new_mut(owner: O, func: F) -> Self {
        SelfMonad {
            owner,
            func,
            phantom: PhantomData
        }
    }
}

impl<O, V: ?Sized, F: Fn(&O) -> &V> Deref for SelfMonad<O, V, F> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        AsRef::as_ref(self)
    }
}

impl<O, V: ?Sized, F: Fn(&O) -> &V> AsRef<V> for SelfMonad<O, V, F> {
    fn as_ref(&self) -> &V {
        (self.func)(&self.owner)
    }
}

impl<O, V: ?Sized, F: FnMut(&mut O) -> &mut V> AsMut<V> for SelfMonad<O, V, F> {
    fn as_mut(&mut self) -> &mut V {
        (self.func)(&mut self.owner)
    }
}

///-------------------------------------------------------------------------------------------------

pub struct SelfMonadMut<O, V: ?Sized, F> {
    owner: O,
    func: RefCell<F>,
    phantom: PhantomData<*const V>
}

impl<O, V: ?Sized, F: FnMut(&O) -> &V> SelfMonadMut<O, V, F> {
    pub fn new(owner: O, func: F) -> Self {
        SelfMonadMut {
            owner,
            func: RefCell::new(func),
            phantom: PhantomData
        }
    }
}

impl<O, V: ?Sized, F: FnMut(&mut O) -> &mut V> SelfMonadMut<O, V, F> {
    pub fn new_mut(owner: O, func: F) -> Self {
        SelfMonadMut {
            owner,
            func: RefCell::new(func),
            phantom: PhantomData
        }
    }
}

impl<O, V: ?Sized, F: FnMut(&O) -> &V> Deref for SelfMonadMut<O, V, F> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        AsRef::as_ref(self)
    }
}

impl<O, V: ?Sized, F: FnMut(&O) -> &V> AsRef<V> for SelfMonadMut<O, V, F> {
    fn as_ref(&self) -> &V {
        (&mut *self.func.borrow_mut())(&self.owner)
    }
}

impl<O, V: ?Sized, F: FnMut(&mut O) -> &mut V> AsMut<V> for SelfMonadMut<O, V, F> {
    fn as_mut(&mut self) -> &mut V {
        (&mut *self.func.borrow_mut())(&mut self.owner)
    }
}

///-------------------------------------------------------------------------------------------------

pub trait SelfMonadOwner<O> {
    fn owner(&self) -> &O;
    fn owner_mut(&mut self) -> &mut O;
    fn owner_into(self) -> O;
}

impl<O, V, F> SelfMonadOwner<O> for SelfMonadOnce<O, V, F> {
    fn owner(&self) -> &O {
        &self.owner
    }

    fn owner_mut(&mut self) -> &mut O {
        &mut self.owner
    }

    fn owner_into(self) -> O {
        self.owner
    }
}

impl<O, V, F> SelfMonadOwner<O> for SelfMonad<O, V, F> {
    fn owner(&self) -> &O {
        &self.owner
    }

    fn owner_mut(&mut self) -> &mut O {
        &mut self.owner
    }

    fn owner_into(self) -> O {
        self.owner
    }
}

impl<O, V, F> SelfMonadOwner<O> for SelfMonadMut<O, V, F> {
    fn owner(&self) -> &O {
        &self.owner
    }

    fn owner_mut(&mut self) -> &mut O {
        &mut self.owner
    }

    fn owner_into(self) -> O {
        self.owner
    }
}

/// Tests ------------------------------------------------------------------------------------------

#[cfg(test)]
mod test_once {
    use crate::SelfMonadOnce;
    use std::borrow::BorrowMut;
    use std::ops::Deref;

    #[test]
    fn once_pointer() {
        let m = SelfMonadOnce::new(String::from("hello"), |s| &s[0..2]);
        assert_eq!("he", &*m);
    }

    #[test]
    fn once_closure() {
        let c: Box<dyn FnOnce(&String) -> &str> = Box::new(|s| &s[0..2]);
        let m = SelfMonadOnce::new(String::from("hello"), c);
        assert_eq!("he", m.deref());
    }

    #[test]
    fn once_as_ref() {
        let m = SelfMonadOnce::new(String::from("hello"), |s| &s[0..2]);
        assert_eq!("he", m.as_ref());
    }

    #[test]
    fn once_closure_as_mut() {
        let c: Box<dyn FnOnce(&mut String) -> &mut str> = Box::new(|s| s[0..2].borrow_mut());
        let mut m = SelfMonadOnce::new_mut(String::from("hello"), c);
        assert_eq!("he", m.as_mut());
    }
}

#[cfg(test)]
mod test {
    use crate::SelfMonad;
    use std::borrow::BorrowMut;

    #[test]
    fn pointer_twice() {
        let m = SelfMonad::new(String::from("hello"), |s| &s[0..2]);
        assert_eq!("he", &*m);
        assert_eq!("he", &*m);
    }

    #[test]
    fn closure() {
        let c: Box<dyn Fn(&String) -> &str> = Box::new(|s| &s[0..2]);
        let m = SelfMonad::new(String::from("hello"), c);
        assert_eq!("he", &*m);
    }

    #[test]
    fn closure_as_mut() {
        let c: Box<dyn Fn(&mut String) -> &mut str> = Box::new(|s| s[0..2].borrow_mut());
        let mut m = SelfMonad::new_mut(String::from("hello"), c);
        assert_eq!("he", m.as_mut());
    }

    #[test]
    fn as_ref() {
        let m = SelfMonad::new(String::from("hello"), |s| &s[0..2]);
        assert_eq!("he", m.as_ref());
    }
}

#[cfg(test)]
mod test_mut {
    use crate::SelfMonadMut;
    use std::borrow::BorrowMut;

    #[test]
    fn mut_pointer_twice() {
        let m = SelfMonadMut::new(String::from("hello"), |s| &s[0..2]);
        assert_eq!("he", &*m);
        assert_eq!("he", &*m);
    }

    #[test]
    fn mut_closure() {
        let c: Box<dyn FnMut(&String) -> &str> = Box::new(|s| &s[0..2]);
        let m = SelfMonadMut::new(String::from("hello"), c);
        assert_eq!("he", &*m);
    }

    #[test]
    fn mut_closure_as_mut() {
        let c: Box<dyn FnMut(&mut String) -> &mut str> = Box::new(|s| s[0..2].borrow_mut());
        let mut m = SelfMonadMut::new_mut(String::from("hello"), c);
        assert_eq!("he", m.as_mut());
    }

    #[test]
    fn mut_as_ref() {
        let m = SelfMonadMut::new(String::from("hello"), |s| &s[0..2]);
        assert_eq!("he", m.as_ref());
    }
}
