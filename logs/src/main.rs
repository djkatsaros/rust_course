/*
* Read and parse a log file, return the errors.
*
* logs.txt is a sample fake logging data file
*/

use std::fs; // file system
use std::io::Error;

    // use Result enum to handle errors: 
    // enum Result<T, E> {
    //     Ok(value).
    //     Err(error) }
    // Ex:

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n"); // returns a vector of string slices: (&str)'s!!
                                       // doesn't copy text, returns a brand new vector of string
                                       // slices pointing to the text data, pointers in the heap!
    
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());         // copy semantics: slices copied to results vect data in
                                        // heap. Still point to the same place
        }
    }

    results                             // return makes text and split_text go out of scope
                                        // delete these args and anything they own.
}

fn main() -> Result<(), Error> { //main can return a result: -> Result<(), Error>

 //   string_test(
 //     String::from("red"),
 //     &String::from("red"),        // &String and &str both give RO ref to text data
 //     &String::from("red"), //**   // &str refers to text in the Data segm without a heap
                                     // allocation -> faster. 
                                     // &str also allows us to take a portion of text that is 
                                     // already on the heap ( and already owned by another
                                     // binding)
                                     // **Rust automatically turns &String into &str behind the
                                     // curtain!!!
                                     // use &str anytime you want to refer a portion of a string
                                     // owned by something else.
 //  );
    let text = fs::read_to_string("logs.txt")?; // ? \equiv try 
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;
    println!(" \n {:#?} \n written to error_logs.txt", error_logs);

    Ok(())
    // ^^^^ replaces match statements, but no print statemetns for failures, just panice
    // Guidelines;
    //  use match or 'if let' stmt ... when youre ready to meaningfully deal with an error
    //  call unwrap(), expect('why panicked') ... for quick debuggin, or if you want to crash 
    //      on an Err()
    //  use try ('?') operator to unwrap or propag Result ... when you dont have any way to 
    //      handle the error in the current fcn.

    // fs::read_to_string returns a Result enum, 
    // match fs::read_to_string("logs.txt") {
    //    Ok(read_text) => {
    //        error_logs = extract_errors(read_text.as_str()); //or &read_text, less explcit
     //                                                            // as rust autom'y makes this
    //                                                             // &str type.
    //        match fs::write("errors.txt",error_logs.join("\n")) {
    //            Ok(()) => println!("Wrote errors.txt"),
    //            Err(reason_write_failed) => {
    //                println!("Writing of errors.txt failed: {}", reason_write_failed);
    //            }
    //        }
                                                   
            
    //    } // If extract errors returns &str,
          // read_text about to go out of scope, so its values will be dropped. The str slices
          // in the error_logs binding point to nothing!
          // returning String means the text is copied in the heap, so when read_text goes
          // out of scope, we still have allocation for this text.
          // Requires extra allocation on the heap! useful if we needed the error_logs 
          // declaration to outlive the match statement!
    //    Err(why_this_failed) => {
    //        println!("Failed to read file: {}", why_this_failed)
    //    }
   // }
    
}

/*
 * 3 areas of memory for rust program compilation:
 *  Stack: Fast, limited size (2-8 MB)
 *
 *  Heap: Slow, but large (GBs size)
 *
 *  Data Segment/ ro(read-only)data segment:  stores literal values we write into our code
 *      let num = 45;
 *      let color = "red";
 *
 *  Ex:
 *  let nums = vec![1,2,3,4,5]
 *  Common pattern: Stack stores metadata about a datastructure
 *                  Heap stores the actual data
 *                  -> avoids running out of memory in the stack if the data structure grows too
 *                      large.
 *  for nums, in the...
 *      Stack: Vec Struct [ pointer to values in the Heap | length (5) | capacity (7) ]
*       Heap: 1, 2, 3, 4, 5 *metadata placed here if we have a nested structure: say vec_of_nums=
*       vec![ vec![1,2,3,4,5] ]. In the stack is a pointer to hte inner vector, and the meta data
*       for the inner vec is in the Heap
*       Data: 1, 2, 3, 4, 5 literal values
*
*  for String, &String and &str (string slice)...
*       Let color = String::from("red");
*           Stack: Struct with [pointer to text in the heap | length of string in the heap | capacity of string
*           in the heap]
*           Heap: "red", with <- from stack point pointed to the heap
*           Data: "red"
*       Lert color = &String::from("red");
*           Stack: RO Reference to String struct [ptr to text in heap | length of string in heap |
*           capacity of string in heap]
*           Heap: "red" <- pointer
*           Data: "Red"
*       let name = "me"; or let color = String::from("red").as_str;
*           Stack: &str [pointer to the text in Data (!) | length of string]
*           Data: "red"
*       let c = color.as_str();
*           Stack: String Struct [pointer to "red" on the Heap | length in heap| capacity in heap]
*           and a &str [pointer to text on the Heap (!) | length of string in heap]
*           Heap: "red" <- pointer from &str and <- pointer from string struct
*           Data: "red"
 */ 

