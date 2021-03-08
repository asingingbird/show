use clap::{App, ArgMatches};
use std::path::{Component, Path, Prefix, PathBuf};

pub trait UtilSubCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b>;
    fn run(args: &ArgMatches);
}

/// Converts a windows style path name to unix style path name
pub trait PathExt {
    fn to_absolute(&self) -> PathBuf;
    fn to_unix_style(&self) -> String;
    fn is_symlink(&self) -> bool;
    fn is_executable(&self) -> bool;
}

impl PathExt for Path {
    fn to_absolute(&self) -> PathBuf {
        PathBuf::from(&self)
    }
    fn to_unix_style(&self) -> String {
        if cfg!(windows) {
            let mut path = String::with_capacity(self.as_os_str().len());
            for comp in self.components() {
                match comp {
                    Component::Prefix(prefix_component) => {
                        match prefix_component.kind() {
                            Prefix::Verbatim(p) => path.push_str(&p.to_string_lossy()),
                            Prefix::VerbatimUNC(server, share) => {
                                path.push_str(&server.to_string_lossy());
                                path.push('/');
                                path.push_str(&share.to_string_lossy());
                            }
                            Prefix::VerbatimDisk(disk) => {
                                path.push(disk as char);
                                path.push(':');
                            }
                            Prefix::DeviceNS(dev) => path.push_str(&dev.to_string_lossy()),
                            Prefix::UNC(server, share) => {
                                path.push_str(&server.to_string_lossy());
                                path.push('/');
                                path.push_str(&share.to_string_lossy());
                            }
                            Prefix::Disk(disk) => {
                                path.push(disk as char);
                                path.push(':');
                            }
                        }
                        // Prefix does not need append `/`
                        continue;
                    }
                    Component::RootDir => {}
                    Component::CurDir => path.push_str("."),
                    Component::ParentDir => path.push_str(".."),
                    Component::Normal(p) => path.push_str(&p.to_string_lossy()),
                }
                path.push('/');
            }
            // Trim the last '/'
            if path.len() > 1 && path.ends_with('/') {
                path.pop();
            }
            path
        } else {
            self.to_string_lossy().to_string()
        }
    }

    #[inline]
    fn is_symlink(&self) -> bool {
        if let Ok(meta) = self.symlink_metadata() {
            meta.file_type().is_symlink()
        } else {
            false
        }
    }

    fn is_executable(&self) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            return self.metadata().map_or(false, |meta| {
                meta.is_file() && meta.mode() & 0o111 != 0
            });
        }

        #[cfg(windows)]
        {
            use std::env;
            use std::os::windows::ffi::OsStrExt;
            use winapi::um::winbase::GetBinaryTypeW;

            // Check file existence first
            if !self.is_file() {
                return false;
            }
            // Check file extension
            if let Some(exec) = std::env::var_os("PATHEXT") {
                if let Some(extension) = self.extension() {
                    return env::split_paths(&exec)
                        .map(|e| e.to_string_lossy().to_string())
                        // Remove the leading `.`
                        .any(|s| extension.to_string_lossy().eq_ignore_ascii_case(&s[1..]));
                }
            }

            // Check file properties if no file extensions
            let win_path = self
                .as_os_str()
                .encode_wide()
                // Add the final null terminator
                .chain(Some(0))
                .collect::<Vec<u16>>()
                .as_ptr();

            let mut binary_type: u32 = 42;
            let binary_type_ptr = &mut binary_type as *mut u32;
            unsafe { GetBinaryTypeW(win_path, binary_type_ptr) != 0 }
        }
    }
}
