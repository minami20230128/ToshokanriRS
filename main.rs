pub mod book;
pub mod bookshelf;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let menu = menu();
    match menu//switch的な
    {
        1=>show_allbooks(),//関数名はスネークケースの方がいいらしい
        2=>show_allbooks_by_author(),
        3=>search_books_by_title(),
        4=>search_books_by_author(),
        5=>add_books(),
        6=>delete_books(),
        7=>save_books(),
        8=>load_books(),
        9=>modify_book(),
        _=>println!("1~9の数字を入力して下さい"),//1~9に当てはまらないパターンを書かないとエラーになる
    }
}

fn menu() ->i32 {//返り値は矢印で書く
    println!("1, 全ての書籍を見る");
    println!("2, 著者順に書籍を見る");
    println!("3, 書籍を検索する");
    println!("4, 書籍を著者名から検索する");
    println!("5, 書籍を追加する");
    println!("6, 書籍を削除する");
    println!("7, 情報を保存する");
    println!("8, 情報を読み込む");
    println!("9, 情報を修正する");
    let mut strmenu: String = String::from("");
    io::stdin().read_line(&mut strmenu).expect("Failed to read line");//入力するときはいったん文字列でやってあとで変換
    let menu = strmenu.trim().parse().ok().unwrap();
    return menu;
}

fn show_allbooks(){

}

fn show_allbooks_by_author(){

}

fn search_books_by_title(){

}

fn search_books_by_author(){

}

fn add_books(){

}

fn delete_books(){

}

fn save_books(){

}

fn load_books(){
    let mut filename: String = String::from("");
    println!("ファイル名を入力して下さい");
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents);
    for line in contents.lines(){
        let bookinfos: Vec<&str> = line.split(',').collect();
        let mut iter = bookinfos.iter();
        let title = iter.next();
        let publisher = iter.next();
        let date = iter.next();
        let mut authors = Vec::new();
        while(iter.next() == None)
        {
            authors.push(iter.next());
        }

        let book = book::Book{
            title : title.as_str(), 
            publisher = publisher, 
            date = date, 
            authors = authors
        };
    }

}

fn modify_book(){

}