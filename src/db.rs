use crate::prelude::*;
use ::tar::{Archive, Entries, Entry};
use flate2::read::GzDecoder;
use std::fs::File;
use std::path::PathBuf;

pub const DEFAULT_DB_DIR: &'static str = "/var/lib/pacman";

/// `Database` encapsulates the functionality for packages
pub struct Database {
    name: String,  // Name of the database to load e.g. `core`
    dir: PathBuf,  // Directory to search for databases e.g. `/var/lib/pacman`
    path: PathBuf, // Full path to the database e.g. `/var/lib/pacman/sync/core.db`
}
impl Database {
    /// Set the directory to search for databases
    pub fn with_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.dir = dir.into();
        self
    }

    /// Load a database by name using the default directory. To control where the
    /// database is loaded from use `Database::default()` and the builder pattern
    /// to call `with_` functions to set the desired properties.
    ///
    /// Default: loads from /var/lib/pacman
    pub fn load(name: &str) -> Result<Self> {
        let mut db = Self {
            name: name.to_owned().to_lowercase(),
            ..Default::default()
        };
        db.path = db.dir.join("sync").join(format!("{}.db", db.name));

        // Validate the database exists
        if db.path.is_file() {
            return Err(Error::DatabaseNotFound(db.name));
        }

        Ok(db)
    }

    /// Construct an iterator over the packages in this database
    pub fn packages(&self) -> Result<Packages> {
        Ok(Packages {
            path: PathBuf::from(&self.path),
            archive: None,
            entries: None,
        })
    }
}

/// Use `Database::default()` and the builder pattern to call various `with_` options
/// to control the exact database initialization desired.
impl Default for Database {
    fn default() -> Self {
        Self {
            name: Default::default(),
            dir: PathBuf::from(DEFAULT_DB_DIR),
            path: Default::default(),
        }
    }
}

/// Iterator for Packages
pub struct Packages<'a> {
    path: PathBuf,
    archive: Option<Archive<GzDecoder<File>>>,
    entries: Option<Entries<'a, GzDecoder<File>>>,
}

impl<'a> Iterator for Packages<'a> {
    type Item = Result<Package>;

    /// Iterator on the tokens
    fn next(&mut self) -> Option<Self::Item> {
        // Construct the iterator if needed
        if self.entries.is_none() {
            if let Ok(f) = File::open(&self.path) {
                self.archive = Some(Archive::new(GzDecoder::new(f)));
            } else {
                return Some(Err(Error::DatabaseLoad(PathBuf::from(&self.path))));
            }
            // self.entries = Some(self.archive.as_ref().entries());
            if let Some(&archive) = self.archive {
                // if let Ok(entries) = archive.entries() {
                //     self.entries = Some(entries)
                // }
            }
        }

        // match self.iter.next() {
        //     None => None,
        //     Some(x) => {
        //         // Parse the package from the iterated tarball file
        //     },
        // }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use ::tar::Archive;
    use flate2::read::GzDecoder;
    use std::io::Read;
    use std::{convert::TryFrom, fs::File};

    #[test]
    fn test_load_community_db() {
        let pkg_name = "supertux";
        let f = File::open("tests/sync/community.db").unwrap();
        let mut archive = Archive::new(GzDecoder::new(f));

        // Decompress target file into memory
        for mut entry in archive.entries().unwrap().filter_map(|x| x.ok()) {
            let path = entry.path().unwrap();
            let path_str = path.to_str().unwrap();

            // Extract target package file as a string
            if path_str.starts_with(&format!("{}-", pkg_name)) && path_str.ends_with("/desc") {
                let mut buffer = Vec::new();
                entry.read_to_end(&mut buffer).unwrap();
                let desc = std::str::from_utf8(&buffer).unwrap();

                let pkg = Package::try_from(desc).unwrap();
                println!("{}", pkg);
            }
        }
    }
}
