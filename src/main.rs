use std::io::{self, Write};
use lab3_new::Library;

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
