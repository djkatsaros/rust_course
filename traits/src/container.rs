
/* generic traits */

pub trait Container<T> {
    fn get(&mut self) -> Option<T>; // left 'abstract' to rely on the basket and the stack to 
                                    // implement their own get method. No method body
    fn put(&mut self, item: T);
    fn is_empty(&self) -> bool;
}
