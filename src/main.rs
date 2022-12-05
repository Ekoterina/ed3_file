use std::fs::File;
use std::fs;
use std::io::Read;

//Функция открывает файл, читает его в строку и возвращает, при неудачной попытке
//открыть или прочитать файл выполняет проброс ошибки(std::io::Error).
fn read_file(path: &str) -> Result <String, std::io::Error> {
    let mut text = String::new();
    File::open(path)?.read_to_string(&mut text)?;
    Ok(text)
}

//Функция открывает и читает файл в строку, возвращает считанную строку, при ошибках
//открытия или чтения генерирует панику.
fn read_file_exact(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

//Распечатать содержимое файла при помощи функции ‘read_file’, при ошибке вывести
//сообщение, что прочитать файл не удалось и уточнить почему. Распечатать содержимое
//файла при помощи функции ‘read_file_exact’.
fn main() {
    let file = "text.txt";
    let file2 = "text2.txt";

    let text1: String = match read_file(file) {
        Ok(text) => text,
        Err(e) => e.to_string()
    };
    println!("{text1}");

    let text2 = read_file_exact(file2);
    println!("{text2}");
}