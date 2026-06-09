

use std::path::Path;
use lopdf::Document as LopdfDocument;

fn is_pdf(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("pdf"))
            .unwrap_or(false)
}
 
fn is_supported(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("pdf") || e.eq_ignore_ascii_case("epub"))
            .unwrap_or(false)
}
 
fn pdf_title(path: &Path) -> Option<String> {
    let doc = LopdfDocument::load(path).ok()?;
    let obj = doc.trailer.get(b"Info").ok()?;
    let (_, info_obj) = doc.dereference(*obj).ok()?;
    let dict = info_obj.as_dict().ok()?;
    let raw = dict.get(b"Title").ok()?;
    doc.get_object_as_string(*raw).ok()
}

fn parse_page_from_title(title: &str) -> Option<u32> {
    let start = title.rfind('[')?;
    let inner = &title[start + 1..];
    let end   = inner.find(']')?;
    let (lhs, _) = inner[..end].split_once('/')?;
    lhs.trim().parse().ok()
}



