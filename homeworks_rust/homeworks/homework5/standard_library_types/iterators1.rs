// iterators1.rs
//
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
//
// Execute `rustlings hint iterators1` for hints :D

fn main () {
    // vec! is a macro that creates a vector
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // iter() is an iterator method that returns an iterator over the elements of the vector
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

    // next() is an iterator method that returns the next element in the iterator
    // Some() is an option type that wraps the value
    // & is a reference operator
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}
