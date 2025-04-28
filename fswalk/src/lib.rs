#![forbid(unsafe_code)]

use std::io::Read;
use std::path::PathBuf;
use std::{fs, io, path::Path};
////////////////////////////////////////////////////////////////////////////////

type Callback<'a> = dyn FnMut(&mut Handle) + 'a;

#[derive(Default)]
pub struct Walker<'a> {
    callbacks: Vec<Box<Callback<'a>>>,
}

impl<'a> Walker<'a> {
    pub fn new() -> Self {
        Walker { callbacks: vec![] }
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Handle) + 'a,
    {
        self.callbacks.push(Box::new(callback));
    }

    fn determine_changers(&mut self, handler: &mut Handle) -> Vec<&mut Box<Callback<'a>>> {
        self.callbacks
            .iter_mut()
            .fold(vec![], move |mut readers, cb| {
                cb(handler);
                match handler {
                    Handle::File(ref mut handle) => {
                        if handle.wants_to_read {
                            handle.wants_to_read = false;
                            readers.push(cb);
                        }
                    }
                    Handle::Dir(ref mut handle) => {
                        if handle.must_descend {
                            handle.must_descend = false;
                            readers.push(cb);
                        }
                    }
                    _ => {}
                }

                readers
            })
    }

    pub fn walk<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut paths = Vec::new();
        let mut res: io::Result<()> = Ok(());

        if self.callbacks.is_empty() {
            return Ok(());
        }
        match path.as_ref().read_dir() {
            Ok(dir) => {
                dir.for_each(|entry| match entry {
                    Ok(entry) => {
                        paths.push(entry.path());
                    }
                    Err(e) => {
                        res = Err(e);
                    }
                });
            }
            Err(e) => return Err(e),
        }

        for path in paths {
            let handler = &mut determine_handler(&path);
            let changers = self.determine_changers(handler);

            if changers.is_empty() {
                continue;
            }

            match handler {
                Handle::File(_) => {
                    let mut file = fs::File::open(&path)?;
                    let mut content = vec![];
                    match file.read_to_end(&mut content) {
                        Ok(_) => {
                            for cb in changers {
                                cb(&mut Handle::Content {
                                    file_path: &path,
                                    content: content.as_slice(),
                                })
                            }
                        }
                        Err(e) => {
                            res = Err(e);
                        }
                    };
                }
                Handle::Dir(_) => {
                    let mut walker = Walker::new();
                    for cb in changers {
                        walker.add_callback(cb);
                    }
                    match walker.walk(path) {
                        Ok(_) => {}
                        Err(e) => {
                            res = Err(e);
                        }
                    }
                }
                _ => {}
            }
        }
        res
    }
}

fn determine_handler(path: &PathBuf) -> Handle {
    if path.is_file() {
        Handle::File(FileHandle {
            path,
            wants_to_read: false,
        })
    } else if path.is_dir() {
        Handle::Dir(DirHandle {
            path,
            must_descend: false,
        })
    } else {
        unreachable!()
    }
}
////////////////////////////////////////////////////////////////////////////////

pub enum Handle<'a> {
    Dir(DirHandle<'a>),
    File(FileHandle<'a>),
    Content {
        file_path: &'a Path,
        content: &'a [u8],
    },
}

pub struct DirHandle<'a> {
    path: &'a Path,
    must_descend: bool,
}

impl<'a> DirHandle<'a> {
    pub fn descend(&mut self) {
        self.must_descend = true;
    }

    pub fn path(&self) -> &Path {
        self.path
    }
}

pub struct FileHandle<'a> {
    path: &'a Path,
    wants_to_read: bool,
}

impl<'a> FileHandle<'a> {
    pub fn read(&mut self) {
        self.wants_to_read = true;
    }

    pub fn path(&self) -> &Path {
        self.path
    }
}
