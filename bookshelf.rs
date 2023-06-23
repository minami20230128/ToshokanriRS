#[path = "./book.rs"]
mod book;

pub struct Bookshelf
{
    pub booklist: Vec<book::Book>
}

impl Bookshelf{
    pub fn add(&self, book : book::Book)
    {
        self.booklist.push(book);
    }
}