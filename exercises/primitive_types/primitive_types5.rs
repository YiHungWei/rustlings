// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Scroll down for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = (cat.0, cat.1);
    let first = &cat.0[0..5];
    let last = &cat.0[6..];

    println!("{} is {} years old.", name, age);
    println!("first name: {}, last name: {}", first, last);
}






























// Take a look at the Data Types -> The Tuple Type section of the book:
// https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
// Particularly the part about destructuring (second to last example in the section).
// You'll need to make a pattern to bind `name` and `age` to the appropriate parts
// of the tuple. You can do it!!
