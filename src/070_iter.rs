// Theme: Iterators and lifetime parameters for types

enum List {
    Nil,
    Cons(uint, Box<List>)
}

// another representation:
// struct List {
//     data: uint,
//     next: Option<Box<List>>
// }

fn main() {
    let my_list = Cons(1, box Cons(2, box Cons(3, box Nil)));
    let my_list = map(|x| x + 1, &my_list);

    println!("The list has {} elements!", len(&my_list));

    for i in my_list.iter() {
        println!("{}", i);
    }
}

fn len(list: &List) -> uint {
    match *list {
        Nil => 0,
        Cons(_, box ref tail) => 1 + len(tail)
    }
}

fn map(f: |uint| -> uint, list: &List) -> List {
    match *list {
        Nil => Nil,
        Cons(h, box ref t) => Cons(f(h), box map(f, t))
    }
}

struct ListIterator<'a> {
    list: &'a List
}

impl<'a> Iterator<uint> for ListIterator<'a> {
    fn next(&mut self) -> Option<uint> {
        match *self.list {
            Nil => None,
            Cons(h, box ref t) => {
                self.list = t;
                Some(h)
            }
        }
    }
}

impl List {
    fn iter<'a>(&'a self) -> ListIterator<'a> {
        ListIterator { list: self }
    }
}
