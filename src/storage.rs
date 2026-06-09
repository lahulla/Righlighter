
use std::path::{Path, PathBuf};
use std::collections::HashSet;
 
use anyhow::{anyhow, bail, Context, Result};
use hyprland::data::Client;
use hyprland::prelude::HyprDataActiveOptional;
use lopdf::Document as LopdfDocument;
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};
 
use crate::storage::{Document, Highlight};


struct Highlight{
    
    bookname    :  String,
    authorname  :  String,
    page        :  u32,
    selection   :  String,
}

struct Document{
    path: PathBuf,
}

struct Databse;

fn open_database() -> Result<Database>;

fn init_schema() -> Result<()>;

fn build(doc: &Document, selection: String) -> Result<Highlight>


fn insert(
        db : &Database,
        highlight : &Highlight,
    
    ) -> Result<()>;

pub fn search(
    db: &Database,
    query: &str,
) -> Result<Vec<Highlight>>;

pub fn export(
    db: &Database,
    output: &Path,
) -> Result<()>;

