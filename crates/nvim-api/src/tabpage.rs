use std::fmt;

use luajit_bindings::{self as lua, Poppable, Pushable};
use nvim_types::{
    self as nvim,
    Object,
    TabHandle,
};
use serde::{Deserialize, Serialize};

use crate::choose;
use crate::ffi::tabpage::*;
use crate::iterator::SuperIterator;
use crate::Error;
use crate::Window;

/// A wrapper around a Neovim tab handle.
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TabPage(pub(crate) TabHandle);

impl fmt::Debug for TabPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("TabPage").field(&self.0).finish()
    }
}

impl fmt::Display for TabPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl TryFrom<Object> for TabPage {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Ok(TabHandle::try_from(value)?.into())
    }
}

impl From<TabHandle> for TabPage {
    fn from(handle: TabHandle) -> Self {
        Self(handle)
    }
}

impl From<TabPage> for Object {
    fn from(tabpage: TabPage) -> Self {
        tabpage.0.into()
    }
}

impl Poppable for TabPage {
    unsafe fn pop(
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<Self, lua::Error> {
        TabHandle::pop(lstate).map(Into::into)
    }
}

impl Pushable for TabPage {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<std::ffi::c_int, lua::Error> {
        self.0.push(lstate)
    }
}


impl TabPage {
    /// Shorthand for [`get_current_tabpage`](crate::get_current_tabpage).
    #[inline(always)]
    pub fn current() -> Self {
        crate::get_current_tabpage()
    }

    /// Binding to [`nvim_tabpage_del_var`](https://neovim.io/doc/user/api.html#nvim_tabpage_del_var()).
    ///
    /// Removes a tab-scoped (`t:`) variable.
    pub fn del_var(&mut self, name: &str) -> Result<(), Error> {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe { nvim_tabpage_del_var(self.0, name.non_owning(), &mut err) };
        choose!(err, ())
    }

    /// Binding to [`nvim_tabpage_get_number`](https://neovim.io/doc/user/api.html#nvim_tabpage_get_number()).
    ///
    /// Gets the tabpage number.
    pub fn get_number(&self) -> Result<u32, Error> {
        let mut err = nvim::Error::new();
        let number = unsafe { nvim_tabpage_get_number(self.0, &mut err) };
        choose!(err, Ok(number.try_into().expect("always positive")))
    }

    /// Binding to [`nvim_tabpage_get_var`][1].
    ///
    /// Gets a tab-scoped (`t:`) variable.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_tabpage_get_var()
    pub fn get_var<Var>(&self, name: &str) -> Result<Var, Error>
    where
        Var: TryFrom<Object>,
        Error: From<Var::Error>,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        let obj = unsafe {
            nvim_tabpage_get_var(self.0, name.non_owning(), &mut err)
        };
        choose!(err, Ok(Var::try_from(obj)?))
    }

    /// Binding to [`nvim_tabpage_get_win`](https://neovim.io/doc/user/api.html#nvim_tabpage_get_win()).
    ///
    /// Gets the current window in a tabpage.
    pub fn get_win(&self) -> Result<Window, Error> {
        let mut err = nvim::Error::new();
        let handle = unsafe { nvim_tabpage_get_win(self.0, &mut err) };
        choose!(err, Ok(handle.into()))
    }

    /// Binding to [`nvim_tabpage_is_valid`](https://neovim.io/doc/user/api.html#nvim_tabpage_is_valid()).
    ///
    /// Checks if a tabpage is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { nvim_tabpage_is_valid(self.0) }
    }

    /// Binding to [`nvim_tabpage_list_wins`](https://neovim.io/doc/user/api.html#nvim_tabpage_list_wins()).
    ///
    /// Gets the windows in a tabpage.
    pub fn list_wins(&self) -> Result<impl SuperIterator<Window>, Error> {
        let mut err = nvim::Error::new();
        let list = unsafe { nvim_tabpage_list_wins(self.0, &mut err) };
        choose!(
            err,
            Ok({
                list.into_iter().map(|obj| Window::try_from(obj).unwrap())
            })
        )
    }

    /// Binding to [`nvim_tabpage_set_var`](https://neovim.io/doc/user/api.html#nvim_tabpage_set_var()).
    ///
    /// Sets a tab-scoped (`t:`) variable.
    pub fn set_var<Var>(&mut self, name: &str, value: Var) -> Result<(), Error>
    where
        Var: TryInto<Object>,
        Error: From<Var::Error>,
    {
        let mut err = nvim::Error::new();
        let name = nvim::String::from(name);
        unsafe {
            nvim_tabpage_set_var(
                self.0,
                name.non_owning(),
                value.try_into()?.non_owning(),
                &mut err,
            )
        };
        choose!(err, ())
    }
}
