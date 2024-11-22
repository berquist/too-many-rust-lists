use std::mem;

// pub struct Node {
//     elem: i32,
//     next: List,
// }

// pub enum List {
//     Empty,
//     // Elem(i32, Box<List>),
//     More(Box<Node>),
// }

// By doing this, rather than the above, the implementation is kept private.
// However, access to the data itself will need to be done with methods on the
// struct rather than direct struct member access.
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
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
}

impl Drop for List {
    fn drop(&mut self) {
        // We can't drop the contents of the Box after deallocating, so
        // there's no way to drop in a tail-recursive manner!  Instead we're
        // going to have to manually write an iterative drop for `List` that
        // hoists nodes out of their boxes.
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
        // We do this instead of `while let Some(_) = self.pop() {...}`
        // because popping does Copy, and this memcpy can become expensive
        // when the memory footprint/size of the element grows.
        //
        // Not quite, here's a better answer taken from the text:
        //
        // Pop returns Option<i32>, while our implementation only manipulates
        // Links (Box<Node>).  So our implenentation only moves around
        // pointers to nodes, while the pop-based one will move around the
        // values we stored in nodes.  This could be very expensive is we
        // generalize our list and someone uses it to store instances of
        // VeryBigThinkWithADropImpl (VBTWADI).  Box is able to run the drop
        // implementation of its contents in-place, so it doesn't suffer from
        // this issue.  Since VBTWADI is _exactly_ the kind of thing that
        // actually makes using a linked-list desirable over an array,
        // behaving poorly on this case would be a bit of a disappointment.
        //
        // TODO If you wish to have the best of both implementations, you
        // could add a new method, `fn pop_node(&mut self) -> Link`, from
        // which `pop` and `drop` and both be cleanly derived.
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
