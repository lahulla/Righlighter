use std::collections::HashSet;


struct Config{
    library :  PathBuf,
    books   :  HashSet<String>,
}

struct ActiveWindow {
    title   :  String,
}

    
fn active_window() -> Result<ActiveWindow>

fn PRIMARY_SELECTION() -> Result<String>;

fn detect_document(lib: &Path, window: &ActiveWindow,) -> Result<Document> 

fn bookinfo(document: &Document, selection: String) -> Result<Highlight>;


