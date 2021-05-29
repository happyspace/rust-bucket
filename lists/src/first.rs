use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut node_in_box) = cur_link {
            cur_link = mem::replace(&mut node_in_box.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    // #[must_use]
    // just return the reference to the Node 
    // pub fn pop_node(&mut self) -> Link {
    //     let cur_link = mem::replace(&mut self.head, Link::Empty);
    //     if let Link::More(node_in_box) = cur_link {
    //         self.head = node_in_box.next;
    //     } 
    //     cur_link
    // }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        // TODO
        let mut list = List::new();

        // populate list
        list.push(1);
        list.push(2);
        list.push(3);
        
        // check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

       // push some more 
       list.push(4);
       list.push(5);

       // check normal removal
       assert_eq!(list.pop(), Some(5));
       assert_eq!(list.pop(), Some(4));

       // check exhaustion
       assert_eq!(list.pop(), Some(1));
       assert_eq!(list.pop(), None);

       assert_eq!(list.pop(), None);
    }

    #[test]
    fn drop_list() {
        let e_list = List::new();
        println!("create list {:?}", e_list);

        drop(e_list);

        let mut f_list = List::new();
        f_list.push(1);
        f_list.push(2);
        f_list.push(3);

        drop(f_list);


    }
}
