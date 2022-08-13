use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn count() {
    // スマートポインタを作成
    let x = Rc::new(42);
    // 参照カウンタは1
    assert_eq!(Rc::strong_count(&x), 1);

    // ポインタを複製
    let y = x.clone();
    // 参照カウンタは2
    assert_eq!(Rc::strong_count(&y), 2);

    // ２つは同じポインタを指す
    assert!(Rc::ptr_eq(&x, &y));
    eprintln!("x = {0:p} (points to {0:})", x);
    eprintln!("y = {0:p} (points to {0:})", y);

    // カウンタを作成
    // Mutex: 共有書き込みのために必要
    // Arc: ライフタイムに依存しない共有のために必要
    let counter = Arc::new(Mutex::new(0));

    // 以下、２スレッド間でカウンタを共有して作業する
    let thread = thread::spawn({
        let counter = counter.clone();
        move || {
            for _ in 0..100000 {
                // カウンタのロックを取得
                let mut counter = counter.lock().unwrap();
                // 偶数なら１を足す
                if *counter % 2 == 0 {
                    *counter += 1;
                }
            }
        }
    });

    for _ in 0..100000 {
        // カウンタのロックを取得
        let mut counter = counter.lock().unwrap();
        // 偶数なら１を足す
        if *counter % 2 == 0 {
            *counter += 1;
        }
    }

    // スレッドの合流
    thread.join().unwrap();

    // カウンタの最終的な値を取得
    let counter = *counter.lock().unwrap();
    eprintln!("counter = {}", counter);

}
