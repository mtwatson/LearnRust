use list::LinkedList;
use rand::prelude::*;

mod list;

fn main()
{
    let mut rng = rand::thread_rng();

    let mut list = LinkedList::<usize>::new();
    for _ in 1..1000000
    {
        let value = rng.gen::<usize>();
        // list.insert_sorted(value);
    }
}
