use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::io::Write;

fn thread_func(number: char, stop_flag: Arc<AtomicBool>) -> i32 {
    while !stop_flag.load(Ordering::Relaxed) {
        print!("{}", number);
        std::io::stdout().flush().unwrap(); // Чтобы сразу выводить символы на экран
        thread::sleep(Duration::from_secs(1));
    }
    // Код завершения потока
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

    // Ожидаем нажатия клавиши
    println!("Press Enter to stop...");
    let _ = std::io::stdin().read_line(&mut String::new()).unwrap();

    // Устанавливаем флаги завершения для обоих потоков
    stop_flag_1.store(true, Ordering::Relaxed);
    stop_flag_2.store(true, Ordering::Relaxed);

    // Ожидаем завершения потоков и получаем коды завершения
    let exit_code_1 = handle1.join().unwrap();
    let exit_code_2 = handle2.join().unwrap();

    println!("\nThread 1 exited with code: {}", exit_code_1);
    println!("Thread 2 exited with code: {}", exit_code_2);
}

