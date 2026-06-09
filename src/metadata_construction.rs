

use std::collections::HashSet;
use std::path::{Path, PathBuf};
 
use anyhow::{anyhow, bail, Context, Result};
use hyprland::data::Client;
use hyprland::prelude::HyprDataActiveOptional;
use lib_epub::epub::EpubDoc;
use lopdf::Document as LopdfDocument;
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};
 
use crate::storage::{Document, Highlight};

pub struct PDF {
    pub title:  String,
    pub author: String,
    pub page:   u32,
}

pub struct EPUB {
    pub title:  String,
    pub author: String,
}
 
struct Config {
    library: PathBuf,
    books:   HashSet<String>,
}

struct ActiveWindow {
    title: String,
}

fn active_window() -> Result<ActiveWindow> {
    let client = Client::get_active()
        .context("failed to query Hyprland IPC")?
        .ok_or_else(|| anyhow!("no active window"))?;
 
    Ok(ActiveWindow { title: client.title })
}

#[allow(non_snake_case)]
fn PRIMARY_SELECTION() -> Result<String> {
    let (mut reader, _mime) = get_contents(
        ClipboardType::Primary,
        Seat::Unspecified,
        MimeType::Text,
    )
    .map_err(|e| anyhow!("primary selection unavailable: {e}"))?;
 
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut reader, &mut buf)
        .context("failed to read primary selection")?;
 
    if buf.is_empty() {
        bail!("primary selection is empty");
    }
    Ok(buf)
}

pub fn detect_document(lib: &Path, window: &ActiveWindow) -> Result<Document> {
    let needle = window.title.to_lowercase();
 
    for entry in std::fs::read_dir(lib)
        .with_context(|| format!("cannot read library dir: {}", lib.display()))?
    {
        let entry = entry?;
        let path  = entry.path();
 
        if !path.is_file() {
            continue;
        }
 
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
 
        if ext != "pdf" && ext != "epub" {
            continue;
        }
 
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
 
        if needle.contains(&stem) {
            return Ok(Document { path });
        }
    }
 
    bail!(
        "no PDF/EPUB in '{}' matched window title '{}'",
        lib.display(),
        window.title
    )
}

pub fn pdf_metadata(doc: &Document, window: &ActiveWindow) -> Result<PDF> {
    let lopdf = LopdfDocument::load(&doc.path)
        .with_context(|| format!("lopdf failed to open '{}'", doc.path.display()))?;
 
    let info = lopdf
        .trailer
        .get(b"Info")
        .ok()
        .and_then(|obj| lopdf.dereference(*obj).ok())
        .and_then(|(_, obj)| obj.as_dict().ok());
 
    let get_str = |key: &[u8]| -> String {
        info.and_then(|d| d.get(key).ok())
            .and_then(|o| lopdf.get_object_as_string(*o).ok())
            .unwrap_or_default()
    };
 
    let title  = get_str(b"Title");
    let author = get_str(b"Author");
    let page   = parse_page_from_title(&window.title).unwrap_or(1);
 
    Ok(PDF { title, author, page })
}

pub fn epub_metadata(doc: &Document) -> Result<EPUB> {
    let mut epub = EpubDoc::new(&doc.path)
        .map_err(|e| anyhow!("epub open failed: {e}"))?;
 
    let title  = epub.mdata("title").unwrap_or_default();
    let author = epub.mdata("creator").unwrap_or_default();
 
    Ok(EPUB { title, author })
}

pub fn bookinfo(document: &Document, selection: String) -> Result<Highlight> {
    let ext = document
        .path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
 
    match ext.as_str() {
        "pdf" => {
            let window = active_window()?;
            let meta   = pdf_metadata(document, &window)?;
            Ok(Highlight {
                bookname:   meta.title,
                authorname: meta.author,
                page:       meta.page,
                selection,
            })
        }
        "epub" => {
            let meta = epub_metadata(document)?;
            Ok(Highlight {
                bookname:   meta.title,
                authorname: meta.author,
                page:       0,
                selection,
            })
        }
        other => bail!("unsupported document type: '{other}'"),
    }
}



