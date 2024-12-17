//! Contains an API to get a Roblox reflection database using the types
//! from [`rbx_reflection`]. This crate embeds a database for this purpose,
//! but also provides an API for dependents to get a reflection database
//! from a consistent location.
//!
//! The general way this crate should be used is via [`get`]. This method will
//! search for a locally stored reflection database and return it if it's
//! found. If it isn't, it will instead return the bundled one.
//!
//! Additionally, this crate exposes [`get_local`] and [`get_bundled`] for
//! only loading the locally stored database or only the bundled one
//! respectively.
//!
//! ## Local Details
//!
//! This crate will load a reflection database from the file system if one
//! exists in the default location. This location varies upon the OS and is
//! specified here:
//!
//! | OS      | Location                                                            |
//! |:--------|:--------------------------------------------------------------------|
//! | Windows | `%localappdata%/.rbxreflection/database.msgpack`                    |
//! | MacOS   | `$HOME/Library/Application Support/.rbxreflection/database.msgpack` |
//! | Linux   | `$HOME/.rbxreflection/database.msgpack`                             |
//!
//! Additionally, a location override may be specified via the `RBX_DATABASE`
//! environment variable. The `RBX_DATABASE` variable points to the override
//! `database.msgpack` file, _not_ to an override `.rbxreflection` directory.
//!
//! Both the default `database.msgpack` files and any files pointed to by
//! `RBX_DATABASE` must be valid MessagePack serializations of a
//! [`ReflectionDatabase`] if they're present.
mod error;

use rbx_reflection::ReflectionDatabase;

use std::{env, fs, path::PathBuf, sync::OnceLock};

pub use error::Error;

/// An alias to avoid overly verbose types.
type ResultOption<T> = Result<Option<T>, Error>;

static ENCODED_DATABASE: &[u8] = include_bytes!("../database.msgpack");

/// The name of an environment variable that may be used to specify
/// the location of a reflection database to use. The expected format of
/// a file at this point is MessagePack.
pub const OVERRIDE_PATH_VAR: &str = "RBX_DATABASE";

/// The name of the directory used for the local location for a reflection
/// database. The directory will be placed inside the current user's
/// local data folder on MacOS and Windows and inside
/// the home directory on Linux.
pub const LOCAL_DIR_NAME: &str = ".rbxreflection";

lazy_static::lazy_static! {
    static ref BUNDLED_DATABASE: ReflectionDatabase<'static> = {
        log::debug!("Loading bundled reflection database");
        rmp_serde::decode::from_slice(ENCODED_DATABASE).unwrap_or_else(|e| panic!("could not decode reflection database because: {}", e))
    };
}

static LOCAL_DATABASE: OnceLock<ResultOption<ReflectionDatabase<'static>>> = OnceLock::new();

/// Returns a populated [`ReflectionDatabase`]. This will attempt to load one locally and
/// if one can't be found, it will return one that is bundled with this crate.
///
/// ## Errors
///
/// Errors if a locally stored [`ReflectionDatabase`] could not be read
/// or is invalid MessagePack.
pub fn get() -> Result<&'static ReflectionDatabase<'static>, Error> {
    Ok(get_local()?.unwrap_or(&BUNDLED_DATABASE))
}

/// Returns a reflection database from the file system, if one can be found.
/// This is loaded from a location set by the `RBX_DATABASE` environment
/// variable if it's set. Otherwise, the default location is checked.
///
/// The default location varies depending upon OS:
///
/// | OS      | Location                                                            |
/// |:--------|:--------------------------------------------------------------------|
/// | Windows | `%localappdata%/.rbxreflection/database.msgpack`                    |
/// | MacOS   | `$HOME/Library/Application Support/.rbxreflection/database.msgpack` |
/// | Linux   | `$HOME/.rbxreflection/database.msgpack`                             |
///
/// The file at the above location (or the one pointed to by `RBX_DATABASE`)
/// must be valid MessagePack.
///
/// ## Errors
///
/// Errors if the file specified by `RBX_DATABASE` or in the default location
/// exists but is invalid MessagePack.
pub fn get_local() -> ResultOption<&'static ReflectionDatabase<'static>> {
    let inner = LOCAL_DATABASE.get_or_init(|| {
        if let Some(path) = get_local_location() {
            if path.exists() {
                let database: ReflectionDatabase<'static> =
                    rmp_serde::from_slice(&fs::read(path)?)?;
                Ok(Some(database))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    });
    match inner {
        Ok(opt) => Ok(opt.as_ref()),
        // This clone could be avoided because these references are static,
        // but it'd involve some indirection and these errors are rare anyway.
        Err(e) => Err(e.clone()),
    }
}

/// Returns the locally bundled [`ReflectionDatabase`]. This database may or may
/// not be up to date, but it will always exist.
pub fn get_bundled() -> &'static ReflectionDatabase<'static> {
    &BUNDLED_DATABASE
}

/// Fetches the location a [`ReflectionDatabase`] is expected to be loaded from.
/// This may return [`None`] if the local data directory cannot be found.
pub fn get_local_location() -> Option<PathBuf> {
    if let Ok(location) = env::var(OVERRIDE_PATH_VAR) {
        log::debug!("Using environment variable {OVERRIDE_PATH_VAR} to fetch reflection database");
        Some(PathBuf::from(location))
    } else {
        // Due to concerns about the local data directory existing
        // on Linux, we use the home directory instead.
        #[cfg(target_os = "linux")]
        let mut home = dirs::home_dir()?;
        #[cfg(not(target_os = "linux"))]
        let mut home = dirs::data_local_dir()?;

        home.push(LOCAL_DIR_NAME);
        home.push("database.msgpack");
        Some(home)
    }
}

#[cfg(test)]
mod test {
    use rbx_reflection::ClassDescriptor;

    use super::*;

    #[test]
    fn bundled() {
        let _database = get_bundled();
    }

    #[test]
    fn env_var() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("empty.msgpack");

        env::set_var(OVERRIDE_PATH_VAR, &test_path);
        let empty_db = get().unwrap();
        println!("{:?}", empty_db.version);
        assert!(empty_db.version == [0, 0, 0, 0]);
    }

    #[test]
    fn superclasses_iter_test() {
        let database = get_bundled();
        let part_class_descriptor = database.classes.get("Part");
        let mut iter = database.superclasses_iter(part_class_descriptor.unwrap());
        fn class_descriptor_eq(lhs: Option<&ClassDescriptor>, rhs: Option<&ClassDescriptor>) {
            let eq = match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => lhs.name == rhs.name,
                (None, None) => true,
                _ => false,
            };
            assert!(eq, "{:?} != {:?}", lhs, rhs);
        }

        class_descriptor_eq(iter.next(), part_class_descriptor);

        let mut current_class_descriptor = part_class_descriptor.unwrap();
        while let Some(superclass) = current_class_descriptor.superclass.as_ref() {
            let superclass_descriptor = database.classes.get(superclass.as_ref());
            class_descriptor_eq(iter.next(), superclass_descriptor);
            current_class_descriptor = superclass_descriptor.unwrap();
        }

        class_descriptor_eq(iter.next(), None);
    }

    #[test]
    fn has_superclass_test() {
        let database = get_bundled();
        let part_class_descriptor = database.classes.get("Part").unwrap();
        let instance_class_descriptor = database.classes.get("Instance").unwrap();
        assert!(database.has_superclass(part_class_descriptor, instance_class_descriptor));
        assert!(!database.has_superclass(instance_class_descriptor, part_class_descriptor));
    }
}
