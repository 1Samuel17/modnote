
#[derive(Debug, Default)]
pub struct Notebooks {
    collection: String,
    notebooks: Vec<Notebook>,
}

impl Notebooks {
    pub fn new(collection: String) -> Self {
        Self {
            collection,
            notebooks: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Notebook {
    name: String,
    notes: Vec<Note>,
}

impl Notebook {
    // create new notebook, add notes later
    pub fn new(name: String) -> Self {
        Self {
            name,
            notes: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Note {
    topic: String,
    description: String,
    contents: Vec<String>,
    tags: Vec<String>,
}

impl Note {
    // create new note, add contents and tags later
    pub fn new(topic: String, description: String) -> Self {
        Self {
            topic,
            description,
            contents: Vec::new(),
            tags: Vec::new(),
        }
    }
}