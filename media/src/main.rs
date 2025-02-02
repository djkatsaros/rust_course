/*
 * Catalog of 
 *  Books:      title | author
 *  Movies:     title | author
 *  Audiobook:  title
 * 
 *  Use Enums (enumerations)
*/
mod content; // use modules contained in content -> pulls from mod.rs in content
use content::media::Media; // instead of using content::media::Media each time
use content::catalog::Catalog; // vs content::catalog::Catalog each time 

fn print_media(media: Media) {
    println!("{:#?}", media);
}


fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook")
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director")
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author")
    };

    let podcast = Media::Podcast (10); 

    let placeholder = Media::Placeholder {
    };

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(9);
    let placeholder = Media::Placeholder;

    println!("{:#?}", item.unwrap_or(&placeholder) );
    match catalog.get_by_index(0) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("No Value Here");
        }
    }

    // println!("{:#?}", catalog.items.get(0)); // prints "sum" , built in enum 'Option'!
                                             // enum Option {
                                             //     Some(value),
                                             //     None
                                             //     }
    match catalog.items.get(90) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing at that index");
        }
    }
}
