We're going to implement insertion sort by modifying the doubly linked list provided by: [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) chapter 7.

We'll add a method:

    pub fn insert_sorted(&mut self, elem: T)
    {
    }

That will find the appropriate place to add the item and insert it.

Then we'll implement insertion sort with a Vec.

Then we'll profile both to compare their runtimes, and finally discuss reasons for the difference in run times.