use std::collections::HashMap;

use rquickjs::{loader::Loader, Ctx, Error, Module, Object, Result};

pub use self::builder::ModuleLoaderBuilder;
pub use self::global::GlobalInitializer;
pub use self::resolver::ModuleResolver;

mod builder;
mod global;
mod resolver;

type GlobalLoadFn = Box<dyn for<'js> FnOnce(&Ctx<'js>, &Object<'js>) -> Result<()> + Send + Sync>;
type ModuleLoadFn = for<'js> fn(Ctx<'js>, Vec<u8>) -> Result<Module<'js>>;

/// Rquickjs [`Loader`](rquickjs::loader::Loader) for Rust modules
/// defined using [`ModuleDefExt`](crate::ModuleDefExt).
///
/// See [`ModuleLoaderBuilder`] for usage.
pub struct ModuleLoader {
    modules: HashMap<&'static str, ModuleLoadFn>,
}

impl ModuleLoader {
    pub(crate) fn new(modules: HashMap<&'static str, ModuleLoadFn>) -> Self {
        Self { modules }
    }

    pub fn builder() -> ModuleLoaderBuilder {
        ModuleLoaderBuilder::default()
    }
}

impl Loader for ModuleLoader {
    fn load<'js>(&mut self, ctx: &Ctx<'js>, path: &str) -> Result<Module<'js>> {
        let load = self
            .modules
            .remove(path)
            .ok_or_else(|| Error::new_loading(path))?;

        (load)(ctx.clone(), Vec::from(path))
    }
}
