/* Basket: generic struct that can take many data types
 */

use super::container::Container;

pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket <T> {

    pub fn new(item: T) -> Self {
        Basket {item: Some(item)}
    }
}

impl<T> Container<T> for Basket<T> { // T declares the generic types, and second T is a ref to that gen type

    fn get(&mut self) -> Option<T> {
        self.item.take() // takes value out of OPtion and leaves a None value in its place
                         // need this because self.item being called on mutable ref to self,
                         // not allowed to take ownership of item b.c. mut ref
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
