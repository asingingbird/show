use std::collections::BinaryHeap;
use std::fs::{FileType, Metadata};
use std::io;
use std::path::{Path, PathBuf};

pub struct DirEntry {
    path: PathBuf,
    typ: FileType,
    depth: usize,
    follow_link: bool,
    metadata: Metadata,
}

impl DirEntry {
    pub fn from_path<P: AsRef<Path>>(
        path: P,
        depth: usize,
        follow_link: bool,
    ) -> io::Result<DirEntry> {
        let meta = std::fs::metadata(&path)?;
        Ok(DirEntry {
            path: PathBuf::from(path.as_ref()),
            typ: meta.file_type(),
            depth,
            follow_link,
            metadata: meta,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    #[inline]
    pub fn file_type(&self) -> FileType {
        self.typ
    }

    #[inline]
    pub fn is_file(&self) -> bool {
        self.typ.is_file()
    }

    #[inline]
    pub fn is_dir(&self) -> bool {
        self.typ.is_dir()
    }

    #[inline]
    pub fn is_symlink(&self) -> bool {
        self.follow_link || self.typ.is_symlink()
    }

    #[inline]
    pub fn depth(&self) -> usize {
        self.depth
    }

    #[inline]
    pub fn follow_link(&self) -> bool {
        self.follow_link
    }

    #[inline]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[derive(Copy, Clone, Debug)]
pub enum EntryType {
    Directory,
    RegularFile,
    Symlink,
    Pipe,
    Socket,
    Executable,
    Hidden,
    Empty,
}

#[derive(Clone, Debug)]
struct SearchFilter {
    follow_symlink: bool,
    skip_hidden: bool,
    ignore_case: bool,
    min_depth: Option<usize>,
    max_depth: Option<usize>,
    include_pattern: Vec<String>,
    exclude_pattern: Vec<String>,
    create_before: Option<u64>,
    create_after: Option<u64>,
    change_before: Option<u64>,
    change_after:Option<u64>,
    access_before: Option<u64>,
    access_after: Option<u64>,
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self {
            follow_symlink: false,
            skip_hidden: true,
            ignore_case: false,
            min_depth: None,
            max_depth: None,
            include_pattern: vec![],
            exclude_pattern: vec![],
            create_before: None,
            create_after: None,
            change_before: None,
            change_after: None,
            access_before: None,
            access_after: None
        }
    }
}

#[derive(Clone, Debug, Default)]
struct PrintOption {
    ignore_error: bool,
    no_color: bool,
    type_mask: u8,
    absolute_path: bool,
    name_only: bool,
    min_file_size: Option<usize>,
    max_file_size: Option<usize>,
    include_file_extension: Vec<String>,
    exclude_file_extension: Vec<String>,
}

impl PrintOption {
    pub fn new() -> Self {
        Self::default()
    }
}

struct Walk {
    search_paths: Vec<PathBuf>,
    threads: usize,
    filter: SearchFilter,
    option: PrintOption,
}

impl Walk {
    pub fn new() -> Self {
        Self {
            search_paths: Vec::new(),
            threads: 0,
            filter: SearchFilter::default(),
            option: PrintOption::default(),
        }
    }

    pub fn with_threads(mut self, n: usize) -> Self {
        self.threads = n;
        self
    }

    pub fn single_thread(mut self) -> Self {
        self.threads = 1;
        self
    }

    pub fn with_filter(mut self, filter: SearchFilter) -> Self {
        self.filter = filter;
        self
    }

    pub fn with_option(mut self, option: PrintOption) -> Self {
        self.option = option;
        self
    }

    pub fn follow_symlink(mut self, follow_symlink: bool) -> Self {
        self.filter.follow_symlink = follow_symlink;
        self
    }

    pub fn skip_hidden(mut self, skip_hidden: bool) -> Self {
        self.filter.skip_hidden = skip_hidden;
        self
    }

    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.filter.ignore_case = ignore_case;
        self
    }

    pub fn min_depth(mut self, depth: usize) -> Self {
        self.filter.min_depth = Some(depth);
        self
    }

    pub fn max_depth(mut self, depth: usize) -> Self {
        self.filter.max_depth = Some(depth);
        self
    }
}
