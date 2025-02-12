/*
 * Media module
 */

#[derive(Debug)]
pub enum Media { //idea: ~ 3 different structs, all of type Media. Can define fcns of type 'Media'
             // that can take in any of the below
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    Podcast (u32),
    Placeholder
}

impl Media {

    pub fn description(&self) -> String {
        /* // type conditionals
        if let Media::Book {title, author} = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie {title, director} = self {
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook {title} = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media description")
        }*/

        //Pattern matching
        match self {
            Media::Book {title, author} => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie {title, director} => {
                format!("Movie: {} {}", title, director)
            },
            Media::Audiobook {title} => {
                format!("Audiobook: {}", title)
            },
            Media::Podcast (episode_num) => {
                format!("Podcast: {}", episode_num)
            },
            Media::Placeholder => {
                format!("Placeholder")
            }
        }

    }
}
