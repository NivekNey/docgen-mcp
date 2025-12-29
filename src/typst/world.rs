use std::collections::HashMap;

use time::OffsetDateTime;
use typst::Library;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
// The compiler suggested importing LibraryExt from typst::LibraryExt,
// but sometimes it's typst::foundations or elsewhere.
// I'll try typst::LibraryExt based on the suggestion.
// If that fails, I'll try typst::library::LibraryExt.
// Actually, typst 0.14 seems to have it at typst::LibraryExt ?
// Let's try importing it.
// Wait, if I can't find it, I can construct Library manually?
// Library::builder() also comes from LibraryExt.
// Let's assume the compiler suggestion is correct.
use typst::LibraryExt;
use typst::World;

pub struct DocgenWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main: FileId,
    sources: HashMap<FileId, Source>,
    now: OffsetDateTime,
}

impl DocgenWorld {
    pub fn new(source: String) -> Self {
        // Load fonts from typst-assets
        let fonts: Vec<Font> = typst_assets::fonts()
            .flat_map(|bytes| Font::new(Bytes::new(bytes), 0))
            .collect();

        let book = FontBook::from_fonts(&fonts);

        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let mut sources = HashMap::new();
        sources.insert(main_id, Source::new(main_id, source));

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(book),
            fonts,
            main: main_id,
            sources,
            now: OffsetDateTime::now_utc(),
        }
    }
}

impl World for DocgenWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.sources
            .get(&id)
            .cloned()
            .ok_or_else(|| FileError::NotFound(id.vpath().as_rootless_path().into()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        // For now, we don't support external files (images, etc.)
        Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let date = self.now.date();
        Datetime::from_ymd(date.year(), date.month() as u8, date.day())
    }
}
