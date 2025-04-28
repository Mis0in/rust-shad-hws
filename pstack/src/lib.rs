use std::rc::Rc;

pub struct Node<T> {
    value: Rc<T>,
    next: Option<Rc<Node<T>>>,
}

pub struct PStack<T> {
    head: Option<Rc<Node<T>>>,
    size: usize,
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        PStack {
            head: self.head.clone(),
            size: self.size,
        }
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        PStack {
            head: None,
            size: 0,
        }
    }

    pub fn push(&self, value: T) -> Self {
        PStack {
            head: Some(Rc::new(Node {
                value: Rc::new(value),
                next: self.head.clone(),
            })),
            size: self.size + 1,
        }
    }

    pub fn pop(&self) -> Option<(Rc<T>, Self)> {
        self.head.as_ref().map(|node| {
            (
                node.value.clone(),
                PStack {
                    head: node.next.clone(),
                    size: self.size - 1,
                },
            )
        })
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn iter(&self) -> impl Iterator<Item = Rc<T>> {
        let mut current = self.head.clone();
        std::iter::from_fn(move || {
            if let Some(node) = current.clone() {
                current = node.next.clone();
                Some(node.value.clone())
            } else {
                None
            }
        })
    }
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        PStack::new()
    }
}
