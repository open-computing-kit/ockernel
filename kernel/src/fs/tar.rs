//! simple ustar parser

use crate::process::Buffer;

use super::kernel::FileDescriptor;
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use async_trait::async_trait;
use common::{Errno, OpenFlags};
use core::{ffi::CStr, fmt, mem::size_of, str};
use generic_array::{
    typenum::{U12, U8},
    ArrayLength, GenericArray,
};
use log::error;

pub type UserID = usize;
pub type GroupID = usize;
pub type Permissions = usize;

const BLOCK_SIZE: usize = 512;

/// header of a file in a tar archive. contains many kinds of information about the file
#[repr(C)]
#[derive(Clone)]
pub struct Header {
    name: [u8; 100],
    mode: TarNumber<U8>,
    owner_uid: TarNumber<U8>,
    owner_gid: TarNumber<U8>,
    file_size: TarNumber<U12>,
    mod_time: TarNumber<U12>,
    checksum: TarNumber<U8>,
    kind: EntryKind,
    link_name: [u8; 100],
    ustar_indicator: [u8; 6],
    ustar_version: [u8; 2],
    owner_user_name: [u8; 32],
    owner_group_name: [u8; 32],
    device_major: TarNumber<U8>,
    device_minor: TarNumber<U8>,
    filename_prefix: [u8; 155],
}

fn from_c_str(c: &[u8]) -> &str {
    match CStr::from_bytes_until_nul(c) {
        Ok(string) => string.to_str().unwrap(),
        Err(_) => core::str::from_utf8(c).unwrap(),
    }
}

impl Header {
    pub fn name(&self) -> &str {
        from_c_str(&self.name)
    }

    pub fn mode(&self) -> Permissions {
        Permissions::from(&self.mode)
    }

    pub fn owner_uid(&self) -> UserID {
        UserID::from(&self.owner_uid)
    }

    pub fn owner_gid(&self) -> GroupID {
        GroupID::from(&self.owner_gid)
    }

    pub fn file_size(&self) -> usize {
        usize::from(&self.file_size)
    }

    pub fn mod_time(&self) -> usize {
        usize::from(&self.mod_time)
    }

    pub fn checksum(&self) -> usize {
        usize::from(&self.checksum)
    }

    pub fn kind(&self) -> EntryKind {
        self.kind
    }

    pub fn link_name(&self) -> &str {
        from_c_str(&self.link_name)
    }

    pub fn ustar_indicator(&self) -> &str {
        from_c_str(&self.ustar_indicator)
    }

    pub fn ustar_version(&self) -> &str {
        from_c_str(&self.ustar_version)
    }

    pub fn owner_user_name(&self) -> &str {
        from_c_str(&self.owner_user_name)
    }

    pub fn owner_group_name(&self) -> &str {
        from_c_str(&self.owner_group_name)
    }

    pub fn device_major(&self) -> usize {
        usize::from(&self.device_major)
    }

    pub fn device_minor(&self) -> usize {
        usize::from(&self.device_minor)
    }

    pub fn filename_prefix(&self) -> &str {
        from_c_str(&self.filename_prefix)
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("name", &self.name())
            .field("mode", &self.mode())
            .field("owner_uid", &self.owner_uid())
            .field("owner_gid", &self.owner_gid())
            .field("file_size", &self.file_size())
            .field("mod_time", &self.mod_time())
            .field("checksum", &self.checksum())
            .field("kind", &self.kind())
            .field("link_name", &self.link_name())
            .field("ustar_indicator", &self.ustar_indicator())
            .field("ustar_version", &self.ustar_version())
            .field("owner_user_name", &self.owner_user_name())
            .field("owner_group_name", &self.owner_group_name())
            .field("device_major", &self.device_major())
            .field("device_minor", &self.device_minor())
            .field("filename_prefix", &self.filename_prefix())
            .finish()
    }
}

impl TryFrom<&Header> for common::FileStat {
    type Error = common::Errno;

    fn try_from(header: &Header) -> Result<Self, Self::Error> {
        let mode: u16 = header.mode().try_into().map_err(|_| Errno::ValueOverflow)?;
        let mod_time = header.mod_time().try_into().map_err(|_| Errno::ValueOverflow)?;
        Ok(common::FileStat {
            device: 0,
            serial_num: 0,
            mode: common::FileMode {
                permissions: mode.into(),
                kind: header.kind().try_into().unwrap_or_default(),
            },
            num_links: 0,
            user_id: header.owner_uid().try_into().map_err(|_| Errno::ValueOverflow)?,
            group_id: header.owner_gid().try_into().map_err(|_| Errno::ValueOverflow)?,
            size: header.file_size().try_into().map_err(|_| Errno::ValueOverflow)?,
            access_time: mod_time,
            modification_time: mod_time,
            status_change_time: mod_time,
            block_size: 0,
            num_blocks: 0,
        })
    }
}

impl TryFrom<Header> for common::FileStat {
    type Error = common::Errno;

    fn try_from(header: Header) -> Result<Self, Self::Error> {
        (&header).try_into()
    }
}

/// representation of a number in a tar file
#[derive(Clone)]
struct TarNumber<N: ArrayLength<u8>> {
    data: GenericArray<u8, N>,
}

impl<N: ArrayLength<u8>> TarNumber<N> {
    fn to_str(&self) -> &str {
        // get length of string. numeric values are supposed to end in either a null byte or a space and we don't want rust tripping over those values
        let length = self.data.iter().position(|c| *c == 0 || *c == 32).unwrap_or(self.data.len());

        // convert the raw bytes into a string
        core::str::from_utf8(&self.data[0..length]).unwrap()
    }
}

impl<N: ArrayLength<u8>> From<&TarNumber<N>> for usize {
    fn from(num: &TarNumber<N>) -> Self {
        Self::from_str_radix(num.to_str(), 8).unwrap_or(0) //.expect("couldn't parse numeric value")
    }
}

impl<N: ArrayLength<u8>> From<&TarNumber<N>> for u32 {
    fn from(num: &TarNumber<N>) -> Self {
        Self::from_str_radix(num.to_str(), 8).unwrap_or(0) //.expect("couldn't parse numeric value")
    }
}

impl<N: ArrayLength<u8>> fmt::Debug for TarNumber<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: usize = usize::from(self);
        f.debug_struct("TarNumber").field("value", &value).field("raw", &self.data).finish()
    }
}

impl<N: ArrayLength<u8>> From<usize> for TarNumber<N> {
    fn from(value: usize) -> Self {
        let mut data: GenericArray<u8, N> = Default::default();
        let width = data.len();
        data.clone_from_slice(format!("{value:0width$o}").as_bytes());

        Self { data }
    }
}

/// type of file that can be stored in a tar archive
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EntryKind {
    NormalFile = 48,
    HardLink = 49,
    SymLink = 50,
    CharSpecial = 51,
    BlockSpecial = 52,
    Directory = 53,
    FIFO = 54,
    ContiguousFile = 55,
    VendorSpecificA = 65,
    VendorSpecificB = 66,
    VendorSpecificC = 67,
    VendorSpecificD = 68,
    VendorSpecificE = 69,
    VendorSpecificF = 70,
    VendorSpecificG = 71,
    VendorSpecificH = 72,
    VendorSpecificI = 73,
    VendorSpecificJ = 74,
    VendorSpecificK = 75,
    VendorSpecificL = 76,
    VendorSpecificM = 77,
    VendorSpecificN = 78,
    VendorSpecificO = 79,
    VendorSpecificP = 80,
    VendorSpecificQ = 81,
    VendorSpecificR = 82,
    VendorSpecificS = 83,
    VendorSpecificT = 84,
    VendorSpecificU = 85,
    VendorSpecificV = 86,
    VendorSpecificW = 87,
    VendorSpecificX = 88,
    VendorSpecificY = 89,
    VendorSpecificZ = 90,
    GlobalExtendedHeader = 103,
    ExtendedHeaderNext = 120,
}

impl TryFrom<EntryKind> for common::FileKind {
    type Error = ();

    fn try_from(value: EntryKind) -> Result<Self, Self::Error> {
        match value {
            EntryKind::NormalFile | EntryKind::CharSpecial | EntryKind::BlockSpecial | EntryKind::FIFO => Ok(common::FileKind::Regular),
            EntryKind::HardLink | EntryKind::SymLink => Ok(common::FileKind::SymLink),
            EntryKind::Directory => Ok(common::FileKind::Directory),
            _ => Err(()),
        }
    }
}

/// entry in a tar file, as returned by TarIterator
#[derive(Debug)]
pub struct TarEntry<'a> {
    pub header: &'a Header,
    pub contents: &'a [u8],
}

/// struct to enable iterating over a tar file
#[derive(Debug)]
pub struct TarIterator<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> TarIterator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn recreate(&self) -> Self {
        Self::new(self.data)
    }
}

impl<'a> Iterator for TarIterator<'a> {
    type Item = TarEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // make sure we don't overflow the buffer
        if self.offset >= self.data.len() || self.offset + size_of::<Header>() > self.data.len() {
            return None;
        }

        let header = unsafe { &*(self.data.as_ptr().add(self.offset) as *const Header) }; // pointer magic (:

        if header.name().is_empty() {
            return None;
        }

        // make sure the checksum matches
        let checksum = header.checksum();
        let actual_checksum = self.data[self.offset..self.offset + size_of::<Header>()]
            .iter()
            .enumerate()
            .map(|(i, b)| if (148..156).contains(&i) { 32 } else { *b as usize })
            .sum::<usize>();

        if checksum != actual_checksum {
            error!("checksum of tar header ({checksum}) doesn't match calculated checksum ({actual_checksum})");
            return None;
        }

        let file_size = header.file_size();

        let contents_offset = if file_size == 0 {
            self.offset + size_of::<Header>() // dont bother aligning to nearest block if there's no contents, as it just screws things up
        } else {
            ((self.offset + size_of::<Header>()) & !(BLOCK_SIZE - 1)) + BLOCK_SIZE
        };
        let contents_end = contents_offset + file_size;

        self.offset = (contents_end & !(BLOCK_SIZE - 1)) + BLOCK_SIZE;

        Some(TarEntry {
            header,
            contents: &self.data[contents_offset..contents_end],
        })
    }
}

pub fn parse_tar(data: &[u8]) -> TarDirectory {
    let mut root = TarDirectory {
        dir_entries: Vec::new(),
        header: None,
    };

    for entry in TarIterator::new(data) {
        // get full filename if this is ustar
        let filename = if entry.header.ustar_indicator() == "ustar " {
            format!("{}{}", entry.header.filename_prefix(), entry.header.name())
        } else {
            entry.header.name().to_string()
        };

        // split path into its components
        let components = filename.split('/').filter(|name| *name != ".").collect::<Vec<_>>();

        let path;
        let name;

        // get actual filename and path
        if entry.header.kind() == EntryKind::Directory {
            path = &components[..components.len() - 2];
            name = components[components.len() - 2];
        } else {
            path = &components[..components.len() - 1];
            name = components[components.len() - 1];
        }

        // recursively search the built filesystem to add this file or directory
        fn enter_container(path: &[&str], container: &mut TarDirectory, entry: &TarEntry<'_>, filename: &str) {
            let name = if let Some(name) = path.first() {
                name
            } else {
                // add this file/directory to the container and return
                let file = match entry.header.kind() {
                    EntryKind::Directory => DirFile::Directory(TarDirectory {
                        dir_entries: Vec::new(),
                        header: Some(entry.header.clone()),
                    }),
                    EntryKind::SymLink => {
                        let mut header = entry.header.clone();
                        let data: Box<[u8]> = header.link_name().as_bytes().into();
                        header.file_size = data.len().into();

                        DirFile::File(TarFile { data, header })
                    }
                    _ => DirFile::File(TarFile {
                        data: entry.contents.into(),
                        header: entry.header.clone(),
                    }),
                };
                container.dir_entries.push(DirEntry { name: filename.to_string(), file });

                return;
            };

            let new_container = container.dir_entries.iter_mut().find(|entry| entry.name == *name);

            if let Some(dir_entry) = new_container {
                match &mut dir_entry.file {
                    DirFile::File(_) => panic!("can't treat a file as a directory"),
                    DirFile::Directory(ref mut dir) => enter_container(&path[1..], dir, entry, filename),
                };
            } else {
                let mut new_container = TarDirectory {
                    dir_entries: Vec::new(),
                    header: None,
                };
                enter_container(&path[1..], &mut new_container, entry, filename);
                container.dir_entries.push(DirEntry {
                    name: name.to_string(),
                    file: DirFile::Directory(new_container),
                });
            }
        }

        enter_container(path, &mut root, &entry, name);
    }

    root
}

pub struct TarFile {
    data: Box<[u8]>,
    header: Header,
}

impl Clone for TarFile {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            header: self.header.clone(),
        }
    }
}

#[async_trait]
impl FileDescriptor for TarFile {
    async fn read(&self, position: i64, buffer: Buffer) -> common::Result<usize> {
        let position = position.try_into().map_err(|_| Errno::ValueOverflow)?;
        let end: usize = position + buffer.len();
        buffer.copy_from(&self.data[position..end.min(self.data.len())]).await
    }

    async fn stat(&self) -> common::Result<common::FileStat> {
        (&self.header).try_into()
    }
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)] // i think clippy is confused
enum DirFile {
    File(TarFile),
    Directory(TarDirectory),
}

#[derive(Clone)]
struct DirEntry {
    name: String,
    file: DirFile,
}

pub struct TarDirectory {
    dir_entries: Vec<DirEntry>,
    header: Option<Header>,
}

impl Clone for TarDirectory {
    fn clone(&self) -> Self {
        Self {
            dir_entries: self.dir_entries.clone(),
            header: self.header.clone(),
        }
    }
}

#[async_trait]
impl FileDescriptor for TarDirectory {
    async fn open(&self, name: String, flags: OpenFlags) -> common::Result<Arc<dyn FileDescriptor>> {
        if flags & (OpenFlags::Write | OpenFlags::Create) != OpenFlags::None {
            return Err(Errno::ReadOnlyFilesystem);
        }

        for entry in self.dir_entries.iter() {
            if entry.name == name {
                match entry.file {
                    DirFile::Directory(ref dir) => return Ok(Arc::new(dir.clone())),
                    DirFile::File(ref file) => return Ok(Arc::new(file.clone())),
                }
            }
        }

        Err(Errno::NoSuchFileOrDir)
    }

    async fn read(&self, position: i64, buffer: Buffer) -> common::Result<usize> {
        let position: usize = position.try_into().map_err(|_| Errno::ValueOverflow)?;

        let mut data = Vec::new();

        if position < self.dir_entries.len() {
            let entry = &self.dir_entries[position];
            data.extend_from_slice(&(0_u32.to_ne_bytes()));
            data.extend_from_slice(entry.name.as_bytes());
            data.push(0);
        }

        buffer.copy_from(&data).await
    }

    async fn stat(&self) -> common::Result<common::FileStat> {
        if let Some(header) = self.header.as_ref() {
            header.try_into()
        } else {
            Ok(common::FileStat {
                mode: common::FileMode {
                    permissions: common::Permissions::OwnerRead
                        | common::Permissions::OwnerExecute
                        | common::Permissions::GroupRead
                        | common::Permissions::GroupExecute
                        | common::Permissions::OtherRead
                        | common::Permissions::OtherExecute,
                    kind: common::FileKind::Directory,
                },
                ..Default::default()
            })
        }
    }
}
