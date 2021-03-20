use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    build: u8,
    pre_release_string: String,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut displayed_string = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if self.is_pre_release() {
            displayed_string
                .push_str(format!("-{}.{}", self.pre_release_string, self.build).as_str());
        }
        f.write_str(displayed_string.as_str())?;
        Ok(())
    }
}

pub struct VersionBuilder {
    version: Version,
}

impl Version {
    pub fn not_set() -> Version {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
            build: 0,
            pre_release_string: "".to_string(),
        }
    }

    pub fn with() -> VersionBuilder {
        VersionBuilder::new()
    }

    pub fn build_from(&self) -> VersionBuilder {
        VersionBuilder::build_from(self)
    }

    pub fn is_pre_release(&self) -> bool {
        !self.pre_release_string.is_empty()
    }

    pub fn major(&self) -> u8 {
        self.major
    }

    pub fn minor(&self) -> u8 {
        self.minor
    }

    pub fn patch(&self) -> u8 {
        self.patch
    }

    pub fn build(&self) -> u8 {
        self.build
    }

    pub fn pre_release_string(&self) -> &str {
        self.pre_release_string.as_str()
    }
}

impl VersionBuilder {
    pub fn new() -> VersionBuilder {
        VersionBuilder {
            version: Version::not_set(),
        }
    }

    pub fn build_from(version: &Version) -> VersionBuilder {
        VersionBuilder {
            version: version.clone(),
        }
    }

    pub fn major(mut self, major: u8) -> VersionBuilder {
        self.version.major = major;
        self
    }

    pub fn minor(mut self, minor: u8) -> VersionBuilder {
        self.version.minor = minor;
        self
    }

    pub fn patch(mut self, patch: u8) -> VersionBuilder {
        self.version.patch = patch;
        self
    }

    pub fn pre_release(mut self, pre_release_string: &str, build: u8) -> VersionBuilder {
        self.version.pre_release_string = pre_release_string.to_string();
        self.version.build = build;
        self
    }

    pub fn build(self) -> Version {
        self.version
    }
}
