use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}
type Link<T> = Option<Rc<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            }))
        }
    }
    pub fn tali(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl <T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

// ========= IntoIter ==========
pub struct IntoIter<T> (List<T>);
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().and_then(|node| {
            if let Ok(mut result) = Rc::try_unwrap(node) { // 只有在存在且唯一存在一个强引用 try_unwrap才不会报错
                self.0.head = result.next.take();
                Some(result.elem)
            } else {
                None
            }
        })
    }
}

// ========= Iter ==========
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// =========== IterMut ===========
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let mut next = None;
        if let Some(ref mut rc_node)  = self.head {
            next = Rc::get_mut(rc_node);
        }
        IterMut {next}
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item =&'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().and_then(|node| {
            if let Some(ref mut rc_node) = node.next {
                self.next = Rc::get_mut(rc_node);
                return Some(&mut node.elem);
            }
            Some(&mut node.elem)
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tali();
        assert_eq!(list.head(), Some(&2));

        let list = list.tali();
        assert_eq!(list.head(), Some(&1));

        let list = list.tali();
        assert_eq!(list.head(), None);

        let list = list.tali();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn into_iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new().prepend(23).prepend(2).prepend(3);
        let mut iter = list.iter_mut();
        if let Some(node) = iter.next() {
            *node = 6;
        }
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 23));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        println!("{:?}", list);
    }
}
