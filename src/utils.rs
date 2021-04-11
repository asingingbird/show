use ansi_term::{Color::*, Style};
use clap::{App, ArgMatches};
use log::error;
use memchr::memchr;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

pub trait UtilSubCommand {
    fn util_sub_command<'a, 'b>() -> App<'a, 'b>;
    fn run(args: &ArgMatches);
}

/// Converts a windows style path name to unix style path name
pub trait PathExt {
    fn to_absolute(&self, relative_to: &Path) -> PathBuf;
    fn to_string(&self) -> String;
    fn is_symlink(&self) -> bool;
    fn is_executable(&self) -> bool;
    fn is_binary(&self) -> bool;
}

impl PathExt for Path {
    /// Returns the absolute path relative to `relative_to`, without following symlink,
    /// and removes all `.` and `..`.
    fn to_absolute(&self, relative_to: &Path) -> PathBuf {
        let mut path = PathBuf::new();

        let mut components = self.components();

        if let Some(first_component) = components.next() {
            match first_component {
                Component::Normal(p) => {
                    path.push(relative_to);
                    path.push(p);
                }
                Component::CurDir => {
                    path.push(relative_to);
                }
                Component::ParentDir => {
                    let mut cwd = PathBuf::from(relative_to);
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

        path
    }

    /// Returns the path name as `String`, e.g., use `/` separator instead of `\` on Windows
    fn to_string(&self) -> String {
        if cfg!(windows) {
            use std::path::Prefix;
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
            self.metadata()
                .map_or(false, |meta| meta.is_file() && meta.mode() & 0o111 != 0)
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
                        .map(|e| e.to_string())
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

    /// Returns `true` if this path is a binary file.
    ///
    /// # Note
    /// This function returns false if the path does not exists, or is not a file,
    /// or lack of permission to read from this file.
    ///
    /// # How it works
    /// If a file contains any null byte in its first 1024 bytes, we assume it's a binary file,
    /// this method may not be reliable.
    fn is_binary(&self) -> bool {
        if self.is_file() {
            if let Ok(mut file) = File::open(self) {
                let mut content = [0; 1024];
                if let Ok(bytes_read) = file.read(&mut content) {
                    // Treat PDF format as binary
                    if bytes_read >= 4 && &content[..4] == b"%PDF" {
                        return true;
                    }
                    return memchr(b'\x00', &content[..bytes_read]).is_some();
                }
            }
        }
        false
    }
}

pub fn print_path(path: &Path) {
    let cwd = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            error!("Get current directory failed: {}", e.to_string());
            return;
        }
    };
    let absolute_path = path.to_absolute(&cwd);

    // Do not follow symlink here
    if absolute_path.symlink_metadata().is_err() {
        error!("No such file or directory: {:?}", absolute_path);
        return;
    }

    let path_string = absolute_path.to_string();
    let path_string = if absolute_path.is_dir() {
        Blue.paint(path_string)
    } else if absolute_path.is_executable() {
        Yellow.paint(path_string)
    } else {
        Style::default().paint(path_string)
    };

    if absolute_path.is_symlink() {
        let symlink = absolute_path.read_link().unwrap();

        let mut link_to = absolute_path;
        link_to.pop();
        link_to = symlink.to_absolute(&link_to);
        let colored_link = if link_to.exists() {
            Green.paint(link_to.to_string())
        } else {
            Red.paint(link_to.to_string())
        };
        println!(
            "{} {} {}",
            path_string,
            Cyan.bold().paint("-->"),
            colored_link
        );
    } else {
        println!("{}", path_string);
    };
}
