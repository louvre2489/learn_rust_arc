use std::rc::Rc;

// イミュータブルスタック
#[derive(Debug)]
pub struct Stack<T>(Option<Rc<(T, Stack<T>)>>);

// O(1)コピー
impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn push(&mut self, x: T) {
        let this = Self(self.0.take());
        self.0 = Some(Rc::new((x, this)));
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(rc) = &self.0 {
            Some(&rc.0)
        } else {
            None
        }
    }
}

impl<T: Clone> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        let this = Self(self.0.take());
        if let Some(rc) = this.0 {
            let (head, tail) = Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone());
            *self = tail;
            Some(head)
        } else {
            None
        }
    }
}

pub fn stack() {
    println!("*** Stack Start ***");

    let mut s = Stack::new();
    assert_eq!(s.peek(), None);
    s.push(42);
    assert_eq!(s.peek(), Some(&42));
    assert_eq!(s.pop(), Some(42));
    assert_eq!(s.peek(), None);

    println!("*** Stack End ***");
}
