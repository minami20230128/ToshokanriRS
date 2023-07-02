#[path = "./book.rs"]
pub mod book;

pub struct Bookshelf
{
    pub booklist: Vec<book::Book>
}

impl Bookshelf{
    pub fn add(mut self, book : book::Book)
    {
        self.booklist.push(book);
    }
}