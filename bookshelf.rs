#[path = "./book.rs"]
mod book;

pub struct Bookshelf
{
    pub booklist: Vec<book::Book>
}
