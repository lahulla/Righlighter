
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

