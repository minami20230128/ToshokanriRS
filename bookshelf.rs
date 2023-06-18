#[path = "./book.rs"]
mod book;

pub struct Bookshelf
{
    booklist: Vec<book::Book>
}
