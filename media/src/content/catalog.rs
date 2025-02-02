/*
* catalog implementation
 */
use super::media::Media; // super references parent module

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media> // accesses Media module through super::media::Media stmt
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {items: vec![] }
    }

    pub fn add(&mut self, media: Media) { // want to take ownership of the media binding
        self.items.push(media);                                 //
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // good, something to return
            Option::Some(&self.items[index])
        } else {
            // nothing to return now
            Option::None        }
    }
}

