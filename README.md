# Лабораторная работа 3 – Комплексная разработка консольного приложения на Rust

## Тема лабораторной работы
Разработка системы управления студенческой библиотекой на языке Rust с использованием модульной системы, коллекций, обработки ошибок и сериализации данных.

## Цель работы
Практическое освоение ключевых концепций языка программирования Rust путем создания полноценного консольного приложения.

## Постановка задачи
Разработать консольное приложение "Система управления студенческой библиотекой", которое позволяет:
- Добавлять книги в каталог
- Регистрировать читателей
- Выдавать книги читателям
- Принимать возврат книг
- Просматривать списки книг и читателей
- Сохранять и загружать данные из файла

## Математическая модель
Библиотека = (Книги, Читатели)
Книги = {Книга₁, Книга₂, ..., Книгаₙ}, где Книгаᵢ = (id, название, автор, доступность)
Читатели = {Читатель₁, Читатель₂, ..., Читательₘ}, где Читательⱼ = (id, имя)

## Список идентификаторов

### Модели данных:
| Имя переменной | Тип данных | Смысловое обозначение |
|----------------|------------|-----------------------|
| Book           | struct     | Книга в библиотеке    |
| Reader         | struct     | Читатель библиотеки   |
| id             | u32        | Уникальный идентификатор |
| title          | String     | Название книги        |
| author         | String     | Автор книги           |
| is_available   | bool       | Доступность книги     |
| name           | String     | Имя читателя          |

### Основная библиотека:
| Имя переменной | Тип данных | Смысловое обозначение |
|----------------|------------|-----------------------|
| Library        | struct     | Основная структура библиотеки |
| books          | Vec<Book>  | Список всех книг      |
| readers        | HashMap<u32, Reader> | Читатели по ID |
| next_book_id   | u32        | Следующий ID книги    |
| next_reader_id | u32        | Следующий ID читателя |

### Ошибки:
| Имя переменной | Тип данных | Смысловое обозначение |
|----------------|------------|-----------------------|
| LibraryError   | enum       | Тип ошибок библиотеки |

## Код программы

### Файл: Cargo.toml
```toml
[package]
name = "university_library"
version = "0.1.0"
edition = "2025"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Файл: src/models.rs
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub is_available: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reader {
    pub id: u32,
    pub name: String,
}
```

### Файл: src/lib.rs
```rust
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

pub mod models;
use models::{Book, Reader};

#[derive(Debug)]
pub enum LibraryError {
    BookNotFound,
    BookNotAvailable,
    ReaderNotFound,
    InvalidInput,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::BookNotFound => write!(f, "Книга не найдена."),
            LibraryError::BookNotAvailable => write!(f, "Книга уже выдана."),
            LibraryError::ReaderNotFound => write!(f, "Читатель не найден."),
            LibraryError::InvalidInput => write!(f, "Некорректный ввод."),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    books: Vec<Book>,
    readers: HashMap<u32, Reader>,
    next_book_id: u32,
    next_reader_id: u32,
}

impl Library {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            readers: HashMap::new(),
            next_book_id: 1,
            next_reader_id: 1,
        }
    }

    pub fn add_book(&mut self, title: String, author: String) -> &Book {
        let new_book = Book {
            id: self.next_book_id,
            title,
            author,
            is_available: true,
        };
        self.books.push(new_book);
        self.next_book_id += 1;
        self.books.last().unwrap()
    }

    pub fn register_reader(&mut self, name: String) -> &Reader {
        let new_reader = Reader {
            id: self.next_reader_id,
            name,
        };
        self.readers.insert(new_reader.id, new_reader);
        self.next_reader_id += 1;
        self.readers.get(&(self.next_reader_id - 1)).unwrap()
    }

    pub fn find_book_by_id(&mut self, id: u32) -> Option<&mut Book> {
        self.books.iter_mut().find(|b| b.id == id)
    }

    pub fn borrow_book(&mut self, book_id: u32, reader_id: u32) -> Result<(), LibraryError> {
        if !self.readers.contains_key(&reader_id) {
            return Err(LibraryError::ReaderNotFound);
        }

        let book = self.find_book_by_id(book_id)
            .ok_or(LibraryError::BookNotFound)?;

        if !book.is_available {
            return Err(LibraryError::BookNotAvailable);
        }

        book.is_available = false;
        Ok(())
    }

    pub fn return_book(&mut self, book_id: u32) -> Result<(), LibraryError> {
        let book = self.find_book_by_id(book_id)
            .ok_or(LibraryError::BookNotFound)?;
        book.is_available = true;
        Ok(())
    }

    pub fn list_books(&self) -> &Vec<Book> {
        &self.books
    }

    pub fn list_readers(&self) -> Vec<&Reader> {
        self.readers.values().collect()
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let library = serde_json::from_str(&data)?;
        Ok(library)
    }
}
```

### Файл: src/main.rs
```rust
use std::io::{self, Write};
use university_library::Library;

const DB_FILE: &str = "library_data.json";

fn main() {
    let mut library = match Library::load_from_file(DB_FILE) {
        Ok(lib) => {
            println!("Данные библиотеки загружены.");
            lib
        }
        Err(_) => {
            println!("Файл данных не найден. Создана новая библиотека.");
            Library::new()
        }
    };

    println!("=== БИБЛИОТЕКА УНИВЕРСИТЕТА 2025 ===");
    println!("Система управления книжным фондом\n");

    loop {
        print_menu();
        let choice = read_line().trim().to_string();

        match choice.as_str() {
            "1" => add_book(&mut library),
            "2" => register_reader(&mut library),
            "3" => borrow_book(&mut library),
            "4" => return_book(&mut library),
            "5" => list_books(&library),
            "6" => list_readers(&library),
            "7" => {
                println!("\nЗавершение работы...");
                break;
            }
            _ => println!("\nНеверный выбор. Пожалуйста, выберите пункт от 1 до 7."),
        }

        show_library_memory_address(&library);
    }

    match library.save_to_file(DB_FILE) {
        Ok(_) => println!("Данные успешно сохранены в '{}'.", DB_FILE),
        Err(e) => println!("Ошибка при сохранении данных: {}", e),
    }

    println!("До свидания!");
}

fn print_menu() {
    println!("\n┌──────────────────────────────┐");
    println!("│          ОСНОВНОЕ МЕНЮ        │");
    println!("├──────────────────────────────┤");
    println!("│ 1. Добавить новую книгу      │");
    println!("│ 2. Добавить нового читателя  │");
    println!("│ 3. Выдать книгу читателю     │");
    println!("│ 4. Вернуть книгу             │");
    println!("│ 5. Показать все книги        │");
    println!("│ 6. Показать всех читателей   │");
    println!("│ 7. Выйти из программы        │");
    println!("└──────────────────────────────┘");
    print!("\nВыберите действие: ");
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");
    input
}

fn add_book(library: &mut Library) {
    println!("\n--- ДОБАВЛЕНИЕ НОВОЙ КНИГИ ---");
    print!("Название книги: ");
    io::stdout().flush().unwrap();
    let title = read_line().trim().to_string();
    
    print!("Автор книги: ");
    io::stdout().flush().unwrap();
    let author = read_line().trim().to_string();

    if title.is_empty() || author.is_empty() {
        println!("Ошибка: название и автор не могут быть пустыми.");
        return;
    }

    let book = library.add_book(title, author);
    println!("Книга '{}' (автор: {}) добавлена. ID: {}", 
             book.title, book.author, book.id);
}

fn register_reader(library: &mut Library) {
    println!("\n--- РЕГИСТРАЦИЯ НОВОГО ЧИТАТЕЛЯ ---");
    print!("Имя читателя: ");
    io::stdout().flush().unwrap();
    let name = read_line().trim().to_string();

    if name.is_empty() {
        println!("Ошибка: имя не может быть пустым.");
        return;
    }

    let reader = library.register_reader(name);
    println!("Читатель '{}' зарегистрирован. ID: {}", reader.name, reader.id);
}

fn borrow_book(library: &mut Library) {
    println!("\n--- ВЫДАЧА КНИГИ ---");
    print!("ID книги: ");
    io::stdout().flush().unwrap();
    let book_id: u32 = match read_line().trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Ошибка: некорректный ID книги.");
            return;
        }
    };

    print!("ID читателя: ");
    io::stdout().flush().unwrap();
    let reader_id: u32 = match read_line().trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Ошибка: некорректный ID читателя.");
            return;
        }
    };

    match library.borrow_book(book_id, reader_id) {
        Ok(_) => println!("Книга успешно выдана."),
        Err(e) => println!("Ошибка: {}", e),
    }
}

fn return_book(library: &mut Library) {
    println!("\n--- ВОЗВРАТ КНИГИ ---");
    print!("ID книги: ");
    io::stdout().flush().unwrap();
    let book_id: u32 = match read_line().trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Ошибка: некорректный ID книги.");
            return;
        }
    };

    match library.return_book(book_id) {
        Ok(_) => println!("Книга успешно возвращена."),
        Err(e) => println!("Ошибка: {}", e),
    }
}

fn list_books(library: &Library) {
    let books = library.list_books();

    if books.is_empty() {
        println!("\nВ библиотеке пока нет книг.");
        return;
    }

    println!("\n┌────────────────────────────────────────────────────────────────────┐");
    println!("│                            КАТАЛОГ КНИГ                           │");
    println!("├─────┬────────────────────────────┬──────────────────────┬──────────┤");
    println!("│ ID  │ Название                   │ Автор                │ Статус    │");
    println!("├─────┼────────────────────────────┼──────────────────────┼──────────┤");

    for book in books {
        let status = if book.is_available {
            "Доступна"
        } else {
            "Выдана"
        };
        println!("│ {:3} │ {:<26} │ {:<20} │ {:<8} │",
                 book.id,
                 truncate(&book.title, 26),
                 truncate(&book.author, 20),
                 status);
    }
    println!("└─────┴────────────────────────────┴──────────────────────┴──────────┘");
}

fn list_readers(library: &Library) {
    let readers = library.list_readers();

    if readers.is_empty() {
        println!("\nНет зарегистрированных читателей.");
        return;
    }

    println!("\n┌─────────────────────────────────────────────┐");
    println!("│             РЕГИСТР ЧИТАТЕЛЕЙ               │");
    println!("├─────┬───────────────────────────────────────┤");
    println!("│ ID  │ Имя                                   │");
    println!("├─────┼───────────────────────────────────────┤");

    for reader in readers {
        println!("│ {:3} │ {:<37} │", reader.id, truncate(&reader.name, 37));
    }
    println!("└─────┴───────────────────────────────────────┘");
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}

fn show_library_memory_address(library: &Library) {
    let raw_ptr: *const Library = library;
    
    unsafe {
        println!("\n[Техническая информация]");
        println!("   Адрес объекта в памяти: {:p}", raw_ptr);
    }
}
```

## Результаты выполненной работы

### Создание проекта и сборка:
```
$ cargo new university_library
$ cd university_library
$ cargo build
```

### Запуск приложения:
```
$ cargo run
```

### Выполнение программы:

![Выполнение программы](https://raw.githubusercontent.com/elizabethivanova2412/Rust_lab_3_Ivanova_Elizaveta/main/1.png)
![Выполнение программы](https://raw.githubusercontent.com/elizabethivanova2412/Rust_lab_3_Ivanova_Elizaveta/main/2.png)
![Выполнение программы](https://raw.githubusercontent.com/elizabethivanova2412/Rust_lab_3_Ivanova_Elizaveta/main/3.png)
![Выполнение программы](https://raw.githubusercontent.com/elizabethivanova2412/Rust_lab_3_Ivanova_Elizaveta/main/4.png)
```

```

### Созданный JSON файл (library_data.json):
```json
{
  "books": [
    {
      "id": 1,
      "title": "Программирование на Rust",
      "author": "Кэрол Николс",
      "is_available": false
    }
  ],
  "readers": {
    "1": {
      "id": 1,
      "name": "Петров Сергей"
    }
  },
  "next_book_id": 2,
  "next_reader_id": 2
}
```

## Информация о студенте:
Иванова Елизавета, 1 курс, ПОО
