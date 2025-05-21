// Lifetimes are also needed when structs hold references.
// They are needed to that we know how the lifetime of the
// references within a struct relates to the references of
// the data within it.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
