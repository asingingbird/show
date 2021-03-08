use clap::{App, ArgMatches};
use std::path::{Component, Path, PathBuf, Prefix};
use std::{env, io};

pub trait UtilSubCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b>;
    fn run(args: &ArgMatches);
}

/// Converts a windows style path name to unix style path name
pub trait PathExt {
    fn to_absolute(&self) -> io::Result<PathBuf>;
    #[cfg(windows)]
    fn to_unix_style(&self) -> String;
    fn is_symlink(&self) -> bool;
    fn is_executable(&self) -> bool;
}

impl PathExt for Path {
    /// Returns the absolute path without following symlink, and removes all `.` and `..`
    fn to_absolute(&self) -> io::Result<PathBuf> {
        let mut path = PathBuf::new();

        let mut components = self.components();

        if let Some(first_component) = components.next() {
            match first_component {
                Component::CurDir => {
                    path.push(env::current_dir()?);
                }
                Component::ParentDir => {
                    let mut cwd = env::current_dir()?;
                    // To parent directory
                    cwd.pop();
                    path.push(cwd);
                }
                _ => {
                    path.push(first_component.as_os_str());
                }
            }
        }

        for component in components {
            match component {
                Component::CurDir => {}
                Component::ParentDir => {
                    path.pop();
                }
                _ => {
                    path.push(component.as_os_str());
                }
            }
        }

        Ok(path)
    }

    /// Returns the posix style path name, e.g., use `/` separator instead of `\`
    #[cfg(windows)]
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
                    Component::CurDir => path.push('.'),
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

    /// Returns `true` if this path is symlink
    #[inline]
    fn is_symlink(&self) -> bool {
        if let Ok(meta) = self.symlink_metadata() {
            meta.file_type().is_symlink()
        } else {
            false
        }
    }

    /// Returns `true` if this path is an executable file
    fn is_executable(&self) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            return self
                .metadata()
                .map_or(false, |meta| meta.is_file() && meta.mode() & 0o111 != 0);
        }

        #[cfg(windows)]
        {
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
