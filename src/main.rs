use std::thread;
fn main() {
    // 변수 message는 변경이 불가능하므로, 여러 개의 태스크에서 동시에 접근해도 안전하다.
    let message = "Hello";
    let mut threads = Vec::new();
    // `for` 반복문은 `Iterator` trait 을 구현하는 어떤 객체에 대해서나 사용할 수 있다.
    for num in 0..100000 {
        // `thread::spawn` 을 통해 스레드를 생성한다.
        threads.push(thread::spawn(move || {
            println!("{message} form task {num:?}");
        }));
    }
    // 각 스레드가 끝날때까지 기다린다.
    for thread in threads {
        thread.join().unwrap();
    }
}

