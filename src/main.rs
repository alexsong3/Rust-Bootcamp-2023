#[warn(dead_code)]
fn main() {
    // println!("Hello, Rust Bootcamp by VBI Academy!");

    // let mut numbers = vec![1, 2, 3, 4, 5];
    // println!("111 {:?}", vec![1, 2, 3, 4, 5]);
    // let song = &numbers;
    // let res_number: Vec<_> = reverse_collection(&mut numbers);
    // println!("222 {:?}", res_number);
    // assert_eq!(res_number, vec![5, 4, 3, 2, 1]);

    // let numbers = vec![1, 5, 3, 8, 2];
    //     // assert_eq!(find_max(&numbers), Some(&8));
    //     println!("aaaa {:#?}", find_max(&numbers) == Some(&8));

    // let s = Student {};
    // assert_eq!(s.say_hi(), "hi");
    // assert_eq!(s.say_something(), "I'm a good student");

    let mut stack: Stack<u8> = Stack { items: Vec::new() };
    // println!("222 {:?}", stack.items);
    assert!(stack.items.is_empty());
    stack.insert(1);
    stack.insert(2);
    stack.insert(3);
    assert!(!stack.items.is_empty());
    assert_eq!(stack.remove(), Some(3));
    assert_eq!(stack.remove(), Some(2));
    assert_eq!(stack.remove(), Some(1));
    assert_eq!(stack.remove(), None);
    // assert!(stack.is_empty());
}

// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }

// fn exercise3() {
//     let x = Val { val: 3.0 };
//     let y = Val {
//         val: "hello".to_string(),
//     };
//     println!("{}, {}", x.value(), y.value());
// }

// fn reverse_collection<T: std::fmt::Debug>(collection: &[T]) -> Vec<&T> {
//     // todo!()
//     // collection.iter().rev().collect::<Vec<_>>();
//     // println!("aaaa{:#?}", collection);
//     let res = collection.iter().rev().collect::<Vec<_>>();
//     // println!("bbb {:#?}", res);
//     res
// }

// fn find_max<T: std::cmp::Ord + std::fmt::Debug>(collection: &[T]) -> Option<&T> {
//     // todo!()
//     let res = collection.iter().max();
//     println!("aaaa {:#?}", res);
//     res
// }

// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }

//     fn say_something(&self) -> String;
// }

// //TODO
// #[derive(Debug, PartialEq)]
// struct Student {}
// impl Hello for Student {
//     fn say_hi(&self) -> String {
//         // println!("bbb {:#?}", &self.);
//         // self.say_hi()
//         "hi".to_string()
//         // format!("{:?}", self.say_hi())
//     }

//     fn say_something(&self) -> String {
//         // todo!()
//         "I'm a good student".to_string()
//     }
// }

trait Container {
    type Item;

    fn insert(&mut self, item: Self::Item);
    fn remove(&mut self) -> Option<Self::Item>;
    fn is_empty(&self) -> bool;
}

#[derive(Debug, PartialEq)]
struct Stack<T> {
    items: Vec<T>,
}

//TODO implement Container for Stack
// impl<T> Container for Stack<T> {
impl<T: std::fmt::Debug> Container for Stack<T> {
    // type Item = T;
    type Item = T;
    fn insert(&mut self, item: Self::Item) {
        // todo!()
        // println!("bbb {:#?}", &self);
        self.items.push(item);
    }

    fn remove(&mut self) -> Option<Self::Item> {
        // todo!()
        // println!("ccc {:#?} {:#?}", &self, self.items.len() - 1);
        // println!("ddd {:#?}", Some(3));
        // self.items.remove(&self.items.len() -1)
        let index = self.items.len() - 1;
        // println!("ccc {:#?}", &self.items[item]);
        match self.items.get_mut(index) {
            Some(item) => {
                println!("ddd {:#?}", item);
                // self.items.remove((item);
                // Some(item)
                // Some(&self.items[item])
                todo!()
            }
            None => return None,
        }
        // todo!()
    }
    fn is_empty(&self) -> bool {
        todo!()
    }
}
