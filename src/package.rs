use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Package {
    pub name: String,
    pub base: String,
    pub filename: String,
    pub version: String,
    pub desc: String,
    pub url: String,
    pub size: u64,
    pub isize: u64,
    pub arch: String,
    pub md5sum: String,
    pub sha256sum: String,
    pub pgpsig: String,
    pub build_date: String,
    pub packager: String,
    pub licenses: Vec<String>,
    pub provides: Vec<String>,
    pub depends: Vec<String>,
    pub make_depends: Vec<String>,
    pub optional_depends: Vec<String>,
    pub check_depends: Vec<String>,
}
impl Default for Package {
    fn default() -> Self {
        Self {
            name: Default::default(),
            base: Default::default(),
            filename: Default::default(),
            version: Default::default(),
            desc: Default::default(),
            url: Default::default(),
            size: Default::default(),
            isize: Default::default(),
            arch: Default::default(),
            md5sum: Default::default(),
            sha256sum: Default::default(),
            pgpsig: Default::default(),
            build_date: Default::default(),
            packager: Default::default(),
            licenses: Default::default(),
            provides: Default::default(),
            depends: Default::default(),
            make_depends: Default::default(),
            optional_depends: Default::default(),
            check_depends: Default::default(),
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Base: {}", self.base)?;
        writeln!(f, "Filename: {}", self.filename)?;
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "Desc: {}", self.desc)?;
        writeln!(f, "URL: {}", self.url)?;
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "ISize: {}", self.isize)?;
        writeln!(f, "ARCH: {}", self.arch)?;
        writeln!(f, "md5sum: {}", self.md5sum)?;
        writeln!(f, "sha256sum: {}", self.sha256sum)?;
        writeln!(f, "PGPSig: {}", self.pgpsig)?;
        writeln!(f, "Build Date: {}", self.build_date)?;
        writeln!(f, "Packager: {}", self.packager)?;
        writeln!(f, "Licenses: {:?}", self.licenses)?;
        writeln!(f, "Provides: {:?}", self.provides)?;
        writeln!(f, "Depends: {:?}", self.depends)?;
        writeln!(f, "Make Depends: {:?}", self.make_depends)?;
        writeln!(f, "Optional Depends: {:?}", self.optional_depends)?;
        writeln!(f, "Check Depends: {:?}", self.check_depends)?;
        Ok(())
    }
}
