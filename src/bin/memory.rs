struct  Book { 
    pages : i32,
    rating: i32
}

fn display_page_count (book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating ( book: &Book) {
    println!("rating = {:?}", book.rating);
}

fn main () {
    let book = Book { 
        pages : 50,
        rating: 10
    };

    display_page_count(&book);
    display_rating(&book);
}