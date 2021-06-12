//! A singly linked listed where each element is heap allocated.
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    next: Link<T>,
    data: T,
}

/// A singly linked list.
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    /// Creates a new empty list.
    ///
    /// # Examples
    ///
    /// ```
    /// use list::List;
    ///
    /// let list: List<i32> = List::new();
    /// ```
    pub fn new() -> Self {
        List {
            head: None,
        }
    }

    /// Pushes a new element to the start of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::new();
    ///
    /// list.push(3);
    ///
    /// assert_eq!(3, *list.head().unwrap());
    /// ```
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            data: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /// Pops the first element from the list
    ///
    /// # Examples
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::new();
    ///
    /// list.push(3);
    /// list.push(4);
    ///
    /// let first = list.pop();
    ///
    /// assert_eq!(first.unwrap(), 4);
    ///
    /// let second = list.pop();
    ///
    /// assert_eq!(second.unwrap(), 3);
    /// assert_eq!(None, list.pop());
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    /// Returns the first element of the list
    ///
    /// # Examples
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::new();
    /// assert_eq!(list.head(), None);
    /// list.push(3);
    /// assert_eq!(3, *list.head().unwrap());
    /// ```
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { &node.data
        })
    }

    /// Returns an immutable iterator
    /// over the contents of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use list::List;
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(*iter.next().unwrap(), 2);
    /// assert_eq!(*iter.next().unwrap(), 1);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    /// Returns an mutable iterator
    /// over the contents of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use list::List;
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    ///
    /// let mut iter = list.iter_mut();
    /// *iter.next().unwrap() = 5;
    /// *iter.next().unwrap() = 10;
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(*iter.next().unwrap(), 5);
    /// assert_eq!(*iter.next().unwrap(), 10);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }

    /// Returns an iterator
    /// that will consume all elements of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    ///
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next().unwrap(), 2);
    /// assert_eq!(iter.next().unwrap(), 1);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();

        while let Some(mut node) = curr {
            curr = node.next.take();
        }
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

