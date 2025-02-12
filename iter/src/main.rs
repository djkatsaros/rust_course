


fn print_elements(elements: &[String]) {
    //passing a vector slice:
    //      vec slices, &[type] allow us to point at a portion of data 
    //      currently owned by something else
    //      ** CAN call print_elems with either a full vector OR just a portion
    //      of a vector ** 
    // for loop automatically creates an iterator and calls next() on it
    // and unwraps the option that comes back (Some(...).unwrap())
    // break once next returns a None value
    //for element in elements {
    //    println!("{}", element);
    //}
    elements.iter()
        .for_each(|el| println!("{}", el)); // | | is a closure, anonymous function
    // .iter() makes an iterator, but does nothing (iters are lazyyyy). nothing happens until
    // A) we call 'next'
    // B) you use a function that automatically calls next() -> refered to as iterator consumers
    // .for_each is an iterator consumer
    // .map is an iterator adapter, adds a processing step, but doesn't call the next() function
    // map statement alters the el, needs a consumer to call next()
}

fn shorten_strings(elems: &mut [String]) {
    /* shortens each string in the vector to 1 character 
        modifies existing strings inside of the input vector
        .truncate(int) modifies *in place*
        Takes a mutable reference to the vectir slice of strings -> option to pass
        a vector of strings, or a vector slice.
        */
    elems.iter_mut().for_each(|e| e.truncate(1));
}

fn to_uppercase(elems: &[String]) -> Vec<String> {
    /* returns a new vector of uppercase strings. 
     * accepts a vector slice for flexibility
     */
    elems.iter()
         .map(|el| el.to_uppercase()) // gives new string value, does not modify in place
         .collect()                   // iterator consumer. calls next(). 
                                      // Collect function creeates a Vec<String>, how does
                                      // collect decide this?
                                      // collect() saw we want to return Vec<String>, to decide
                                      // what to return. ONLY works if collect() is an inferred
                                      // return from the fcn
                                      // Also looks at whatever type annotation you put
                                      // in front of a declared variable:
                                      //    let upcased: Vec<String> = elems. ... . collect() 
                                      // Can also call collect like .collect::<Vec<String>>() or
                                      // collect::<Vec<_>>() to rely on rust's type inference
                                      // "Turbofish"
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    /* mov elements out of  vec_a and into vec_b
     * Use into_iter() which gives you ownership of each
     * element *unless called on a ref to a vector* (makes sense, cant 
     * transfer references into ownership?)
     *  examples:
     *      &colors.into_iter() -> iterator created out of a ref -> iterator will prod 
     *      refs to each value 
     *      &mut colors.into_iter() -> iter from mut ref -> iter will prod mut refs to each
     *      value
     *      colors.into_iter() -> iteratro created out of a value -> iter will produce each
     *      value, also moves ownership of these values!
     *      calling into_iter() on these different types/refs mostly allows one to only use
     *      into_iter().
     *      Want ownership of the elements of vec_a -> pass values itself, not a ref
     *      Adding elements to vec_b, make it mutable.
     */
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elems: &[String]) -> Vec<Vec<String>> {
    /* takes each string in input and turn it into its own vector of characters.
     * chars() gives us an iterator from all the individual characters of a string
     *
     */
    elems.iter()
         .map(|el| el.chars()
             .map(|c| c.to_string()
             ).collect()
         ).collect()
}

fn find_color_or(elems: &[String], search: &str, fallback: &str) -> String {
    /*
     * finds search in the elems vector. 
     * returns the fallback if we dont find search.
     * input/return types: 
     *  Search: not storing or changing it, using it in a calculation -> favors RO ref
     *  fallback: not storing or changing, somewhat using in a calc. -> RO ref as a default
     *  return: recieving string refs, but people will use this fcn to take ownership of a 
     *  found slice. If we returned a string slice, our output may go out of scope in use cases 
     */
    elems.iter() //RO refs
            .find(|el| el.contains(search)) // calls next and takes a ref to some elem inside
                                            // vector and places it in the closure fcn
                                            // untl it gets an elem that returns a truthy
                                            // value from the closure fcn
            .map_or(String::from(fallback),  // returns an option, Some(value) if it found
                                             // something and NOne if it didn't.
                    |el| el.to_string())    // map_or belongs to the 
                                            // 'Option' enum. if Option is None, returns first
                                            // arg. If Option is Some(), takes the value
                                            // out of Some and runs it throug h the closure.
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut colors_iter = colors.iter(); // tool (Struct) we use to move through elements of vec
                                         // Iter<string>: [ptr to Data strct we are iterating over
                                         // | pointer to curr pos in the structure
                                         // | pointer to End]
                                         // next() -> Some("red"), moves ptr to "green"
                                         // next() -> Some("green") etc.
                                         // last call to next() -> ptr to end: sign no more 
                                         // elements to iterate over -> None
                                         // mut needed because we are reassigning the pointers!
                                         // == same as changing the iter() Struct!!!!
                                
    //shorten_strings(&mut colors[1..3]);
    //print_elements(&colors);
    //let upper_ = to_uppercase(&colors);
    //let mut destination = vec![];
    //move_elements(upper_, &mut destination);
    //println!("Destination: {:#?}", destination);
    //let uppercased = to_uppercase(&colors);
    //println!("{:#?}", uppercased);
    let found_color = find_color_or(
        &colors,
        "re",
        "Orange"
    );
    println!("{}", found_color);
}

