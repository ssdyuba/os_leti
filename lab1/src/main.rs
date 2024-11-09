use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::io::Write;

fn thread_func(number: char, stop_flag: Arc<AtomicBool>) -> i32 {  //Пришлось использовать атомные
                                                                   //переменные, с которыми пока не
                                                                   //до конца разобрался т.к.
                                                                   //переменные bool нельзя
                                                                   //передавать между потоками
    print!("\nПоток {} начал работу\n", number);
    while !stop_flag.load(Ordering::Relaxed) {
        print!("{}", number);
        std::io::stdout().flush().unwrap();      // Вывод цифр на экран
        thread::sleep(Duration::from_secs(1));
    }
    if number == '1' {
        1
    } else {
        2
    }
}

fn main() {
    let stop_flag_1 = Arc::new(AtomicBool::new(false));
    let stop_flag_2 = Arc::new(AtomicBool::new(false));

    let flag_1 = Arc::clone(&stop_flag_1);
    let flag_2 = Arc::clone(&stop_flag_2);

    let handle1 = thread::spawn(move || thread_func('1', flag_1));
    let handle2 = thread::spawn(move || thread_func('2', flag_2));

    thread::sleep(Duration::from_millis(200)); //Небольшой костыль, чтобы сообщение о клавише Enter
                                               //выводилось после сообщения о запуске потоков
    println!("\nНажмите Enter, чтобы остановить выполнение...");
    let _ = std::io::stdin().read_line(&mut String::new()).unwrap();
    print!("Клавиша нажата");

    stop_flag_1.store(true, Ordering::Relaxed);
    stop_flag_2.store(true, Ordering::Relaxed);

    let exit_code_1 = handle1.join().unwrap();
    let exit_code_2 = handle2.join().unwrap();

    println!("\nПоток {} завершил работу", exit_code_1);
    println!("Поток {} завершил работу", exit_code_2);
}

