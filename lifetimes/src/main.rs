/*
 * Study lifetimes
 * https://www.youtube.com/watch?v=zuWowd9SoXw&pp=ygUabGlmZXRpbWUgbmV0dXJhbCB0ZXJyb3JpdHk%3D
 * jk, how long values live before getting cleaned up automatically as it goes out of scope
 * "help the compiler make sure refs wont outlive the value they refer to"
 *
 */

fn next_language<'a>(langs: &'a [String], current: &str) -> &'a str {
    /*returns the language that comes after the inputed language in the list of languages
    *need to study lifetime of langs! 
    * recall a reference to a val cannot outlive the val it refers to
    * Key: If we have a fcn that takes in two or more refs and 
    * returns a ref, rust makes the assumption that the return ref will point to data referred
    * to by one of the arguments (langs or current in this case).
    * Will also not analyze code body to determine which argument the ref is pointing to!!
    * Important to know because of scope (rule #10). In this case, langs goes out of scope, 
    * current does not.
    * In our case, always returning data relating to the first argument. 
    * Lifetime annotation: rust requires the developer to put in lifetime annotation to
    * identify whether this RO ref is pointing to first or second argument -> <'a> argument
    * declares to rust that there will be a type of reference called a
    * first ref is of type 'a', the returned ref is also of type 'a'
    * 
    * Need to think about annotations anytime fcn receives a ref and returns a ref. 
    * omitted when... only one *ref* input, or
    *               function that takes &self, and any other number of refs and returns a ref
    *               -> rust assumes return ref is tied to &self (good assumption usually). 
    *                   need to annotate if this is not the desired case.
    *           Omit \equiv elide [elision...]
    */
    // first impl
    let mut found = false;
    for lang in langs {
        if found {
            return lang;
        }
        if lang == current {
            found = true;   
        }
    }
    langs.last().unwrap() // .last() returns an Option, Some if we have an el in langs, 
                              // None if not. .unwrap() risks panic if None
} // langs goes out of scope, need to know what reference we are making in &str return

fn last_language(langs: &[String]) -> &str {
    /* return last language in the vec of languages
     * Only one RO argument, so the reference is implicit! Don't need lifetimes.
     */
    langs.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
 /* returns longest language in the inputed list (vec) of languages
  * uses longest() : canonical example for lifetime annotations (too cmplicated?)
  * possible for either of lang_a or lang_b to outlive the function is the idea
  * here -> signal to the reader of the fn signature.
  */
    if lang_a.len() >= lang_b.len() {
        return lang_a;
    } else {
        return lang_b;
    }
}

fn main() {
    let languages = vec![
    String::from("rust"),
    String::from("go"),
    String::from("typescript"),
    ];

    let result = longest_language(&languages[1], &languages[2]); // outputs typescript
    println!("{}", result);

}

/* liftimes referencing example:
 * fn split<'a> (s: &'a str, pattern: &str) -> &'a str
 *
 * just from the function signature, we know that code block A)
 *
 * A)
 * fn main() {
 *  let sentence = "hi how are you";
 *  let result;
 *
 *  {
 *      let pattern = " ";
 *      restul = spliot(sentence, pattern);
 *      } // pattern goes out of scope
 *  println!("{}", result);
 *  }
 *
 *  will work whilst
 *
 *  B) 
 *  fn main() {
 *      let patter = " ";
 *      let result;
 *      {
 *          let sentence = "hi how are you";
 *          result = split(sentence, pattern)
 *          } // sentence goes out of scope
 *      println!("{}", result)
 *  } 
 *  will NOT work because there is a ref to sentence which will not point to anything because
 *  sentence got cleaned up..
 */
