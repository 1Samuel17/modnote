#[derive(Debug, Default, Clone)]
pub struct Collection {
    pub collection_name: String,
    pub description: String,
    pub notebooks: Vec<Notebook>,
}

impl Collection {
    pub fn new(collection_name: String, description: String) -> Self {
        Self { collection_name, description, notebooks: Vec::new() }
    }

    // add a notebook to the collection
    pub fn add_notebook(&mut self, notebook: Notebook) {
        self.notebooks.push(notebook);
    }

    // get a notebook by name, return None if not found
    pub fn get_notebook(&self, name: &str) -> Option<&Notebook> {
        self.notebooks.iter().find(|notebook| notebook.name == name)
    }

    // get all notebooks
    pub fn get_all_notebooks(&self) -> &Vec<Notebook> {
        &self.notebooks
    }
}

#[derive(Debug, Default, Clone)]
pub struct Notebook {
    pub name: String,
    pub notes: Vec<Note>,
}

impl Notebook {
    // create new notebook, add notes later
    pub fn new(name: String) -> Self {
        Self { name, notes: Vec::new() }
    }

    // add a note to the notebook
    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    // get a note by topic, return None if not found
    pub fn get_note(&self, topic: &str) -> Option<&Note> {
        self.notes.iter().find(|note| note.topic == topic)
    }

    // get all notes
    pub fn get_all_notes(&self) -> &Vec<Note> {
        &self.notes
    }
}

#[derive(Debug, Default, Clone)]
pub struct Note {
    pub topic: String,
    pub description: String,
    pub contents: Vec<String>,
    pub tags: Vec<Tag>,
}

impl Note {
    // create new note, add contents and tags later
    pub fn new(topic: String, description: String) -> Self {
        Self { topic, description, contents: Vec::new(), tags: Vec::new() }
    }

    // add contents to note
    pub fn add_content(&mut self, content: String) {
        self.contents.push(content);
    }

    // get all contents
    pub fn get_all_contents(&self) -> &Vec<String> {
        &self.contents
    }

    // get all tags
    pub fn get_all_tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    // add tag to note
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
