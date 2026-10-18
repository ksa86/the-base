use std::collections::HashSet;
use std::io::{self, Write};

// Массив с псевдографикой для каждого этапа игры (от 0 до 6 ошибок)
const HANGMAN_STAGES: [&str; 7] = [
    r#"
       +---+

       |   |
           |
           |
           |
           |
     ========="#, // 0 ошибок
    r#"
       +---+

       |   |
       O   |
           |
           |
           |
     ========="#, // 1 ошибка
    r#"
       +---+

       |   |
       O   |

       |   |
           |
           |
     ========="#, // 2 ошибки
    r#"
       +---+

       |   |
       O   |
      /|   |
           |
           |
     ========="#, // 3 ошибки
    r#"
       +---+

       |   |
       O   |
      /|\  |
           |
           |
     ========="#, // 4 ошибки
    r#"
       +---+

       |   |
       O   |
      /|\  |
      /    |
           |
     ========="#, // 5 ошибок
    r#"
       +---+

       |   |
       O   |
      /|\  |
      / \  |
           |
     ========="#, // 6 ошибок — финальное состояние
];

const SECRET: &str = "blazing";
const MAX_ATTEMPTS: usize = 6;

fn main() {
    let mut guessed_letters: HashSet<char> = HashSet::new();
    let mut all_input_letters: HashSet<char> = HashSet::new();
    let mut attempts_left = MAX_ATTEMPTS;

    println!("--- Добро пожаловать в игру «Виселица»! ---");

    loop {
        println!("\n=========================================");
        
        // Вычисляем количество сделанных ошибок для отрисовки виселицы
        let errors_made = MAX_ATTEMPTS - attempts_left;
        println!("{}", HANGMAN_STAGES[errors_made]);

        // Отображение текущего состояния слова
        print!("\nСлово: ");
        let mut is_won = true;
        for c in SECRET.chars() {
            if guessed_letters.contains(&c) {
                print!("{} ", c);
            } else {
                print!("_ ");
                is_won = false;
            }
        }
        println!();

        // Проверка на победу
        if is_won {
            println!("\nПобеда!");
            break;
        }

        // Проверка на проигрыш
        if attempts_left == 0 {
            println!("\nПоражение(");
            break;
        }

        // Вывод статистики
        println!("Оставшиеся попытки: {}", attempts_left);
        if !all_input_letters.is_empty() {
            let mut sorted_inputs: Vec<char> = all_input_letters.iter().cloned().collect();
            sorted_inputs.sort();
            print!("Использованные буквы: ");
            for c in sorted_inputs {
                print!("{} ", c);
            }
            println!();
        }

        // Чтение ввода пользователя
        print!("Введите букву: ");
        io::stdout().flush().expect("Не удалось очистить буфер вывода");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода");
            continue;
        }

        let input_trimmed = input.trim().to_lowercase();
        let chars: Vec<char> = input_trimmed.chars().collect();

        if chars.len() != 1 {
            println!("введите ровно одну букву.");
            continue;
        }

        let guess = chars[0];

        if !guess.is_alphabetic() {
            println!("Это не буква");
            continue;
        }

        if all_input_letters.contains(&guess) {
            println!("уже вводили букву '{}'", guess);
            continue;
        }

        all_input_letters.insert(guess);

        if SECRET.contains(guess) {
            println!("буква '{}' есть в слове!", guess);
            guessed_letters.insert(guess);
        } else {
            println!("буквы '{}' нет в слове()", guess);
            attempts_left -= 1;
        }
    }
}
