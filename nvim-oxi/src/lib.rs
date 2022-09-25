//! # Rust bindings to all things Neovim
//!
//! The `nvim-oxi` crate provides first-class Rust bindings to the rich API
//! exposed by the [Neovim](https://neovim.io) terminal text editor.
//!
//! The project is mostly intended for plugin authors, although nothing's
//! stopping end users from writing their Neovim configs in Rust.

#![doc(html_root_url = "https://docs.rs/nvim_oxi/0.1.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]

#[doc(hidden)]
pub mod entrypoint;

mod error;
mod toplevel;

pub mod api {
    pub use nvim_api::*;
}

pub mod opts {
    //! Contains all the `*Opts` structs passed to functions as optional
    //! arguments.
    pub use nvim_api::opts::*;
}

pub mod types {
    //! Contains the Rust type definitions of objects given to and returned by
    //! Neovim functions.
    pub use nvim_api::types::*;
}

pub mod lua {
    pub use lua_bindings::*;
}

#[cfg(feature = "libuv")]
#[cfg_attr(docsrs, doc(cfg(feature = "libuv")))]
pub mod libuv {
    pub use nvim_libuv::*;
}

#[cfg(feature = "mlua")]
#[cfg_attr(docsrs, doc(cfg(feature = "mlua")))]
pub mod mlua {
    /// Returns a static reference to a
    /// [`mlua::Lua`](https://docs.rs/mlua/latest/mlua/struct.Lua.html) object
    /// to be able to interact with other Lua plugins.
    pub fn lua() -> &'static mlua::Lua {
        crate::lua::with_state(|lstate| unsafe {
            mlua::Lua::init_from_ptr(lstate as *mut _).into_static()
        })
    }
}

// Re-exports.
pub use error::{Error, Result};
pub use nvim_types::*;
// pub use object::ToObject;
pub use oxi_module::oxi_module as module;
#[cfg(feature = "test")]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub use oxi_test::oxi_test as test;
pub use toplevel::*;
