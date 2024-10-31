use queue::Queue;

pub mod queue;

#[derive(Debug)]
struct Process
{
    id: usize,
    remaining: usize,
    slice: usize,
}

impl Process
{
    pub fn new(id: usize, remaining: usize, slice: usize) -> Self
    {
        Self { id,
               remaining,
               slice }
    }
}

#[derive(Debug, PartialEq)]
struct Stat
{
    id: usize,
    completion: usize,
}

impl Stat
{
    pub fn new(id: usize, completion: usize) -> Self { Self { id, completion } }
}

fn simulate(process_queue: &mut Queue<Process>) -> Vec<Stat>
{
    let mut stats = Vec::<Stat>::new();
    let mut sim_time = 0;

    while let Some(mut process) = process_queue.dequeue()
    {
        let run_time = std::cmp::min(process.remaining, process.slice);
        sim_time += run_time;
        process.remaining -= run_time;
        if process.remaining > 0
        {
            process_queue.enqueue(process);
        }
        else
        {
            stats.push(Stat::new(process.id, sim_time));
        }
    }
    stats
}

fn main()
{
    let mut process_queue = Queue::<Process>::new();
    process_queue.enqueue(Process::new(1, 23, 5));
    process_queue.enqueue(Process::new(2, 29, 7));

    let stats = simulate(&mut process_queue);
    for stat in stats
    {
        println!("{:?}", stat);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_simulate_1()
    {
        let mut process_queue = Queue::<Process>::new();
        process_queue.enqueue(Process::new(1, 23, 5));
        process_queue.enqueue(Process::new(2, 29, 7));

        assert_eq!(simulate(&mut process_queue),
                   vec![Stat::new(1, 51), Stat::new(2, 52)]);
    }

    #[test]
    fn test_simulate_2()
    {
        let mut process_queue = Queue::<Process>::new();
        process_queue.enqueue(Process::new(1, 9, 9));
        process_queue.enqueue(Process::new(2, 7, 7));
        process_queue.enqueue(Process::new(3, 6, 7));
        process_queue.enqueue(Process::new(4, 1, 10));
        process_queue.enqueue(Process::new(5, 8, 8));
        process_queue.enqueue(Process::new(6, 45, 9));
        process_queue.enqueue(Process::new(7, 53, 10));
        process_queue.enqueue(Process::new(8, 11, 10));
        process_queue.enqueue(Process::new(9, 9, 10));
        process_queue.enqueue(Process::new(10, 2, 6));

        assert_eq!(simulate(&mut process_queue),
                   vec![Stat::new(1, 9),
                        Stat::new(2, 16),
                        Stat::new(3, 22),
                        Stat::new(4, 23),
                        Stat::new(5, 31),
                        Stat::new(9, 69),
                        Stat::new(10, 71),
                        Stat::new(8, 91),
                        Stat::new(6, 138),
                        Stat::new(7, 151),]);
    }

    #[test]
    fn test_simulate_3()
    {
        let mut process_queue = Queue::<Process>::new();
        process_queue.enqueue(Process::new(1, 6, 11));
        process_queue.enqueue(Process::new(2, 3, 5));
        process_queue.enqueue(Process::new(3, 18, 6));
        process_queue.enqueue(Process::new(4, 5, 5));
        process_queue.enqueue(Process::new(5, 3, 7));
        process_queue.enqueue(Process::new(6, 14, 7));
        process_queue.enqueue(Process::new(7, 3, 3));
        process_queue.enqueue(Process::new(8, 6, 10));
        process_queue.enqueue(Process::new(9, 29, 5));
        process_queue.enqueue(Process::new(10, 7, 7));

        assert_eq!(simulate(&mut process_queue),
                   vec![Stat::new(1, 6),
                        Stat::new(2, 9),
                        Stat::new(4, 20),
                        Stat::new(5, 23),
                        Stat::new(7, 33),
                        Stat::new(8, 39),
                        Stat::new(10, 51),
                        Stat::new(6, 64),
                        Stat::new(3, 75),
                        Stat::new(9, 94),]);
    }

    #[test]
    fn test_simulate_4()
    {
        let mut process_queue = Queue::<Process>::new();
        process_queue.enqueue(Process::new(1, 15, 3));
        process_queue.enqueue(Process::new(2, 3, 10));
        process_queue.enqueue(Process::new(3, 6, 6));
        process_queue.enqueue(Process::new(4, 6, 6));
        process_queue.enqueue(Process::new(5, 4, 4));
        process_queue.enqueue(Process::new(6, 2, 9));
        process_queue.enqueue(Process::new(7, 7, 7));
        process_queue.enqueue(Process::new(8, 2, 3));
        process_queue.enqueue(Process::new(9, 59, 11));
        process_queue.enqueue(Process::new(10, 2, 8));

        assert_eq!(simulate(&mut process_queue),
                   vec![Stat::new(2, 6),
                        Stat::new(3, 12),
                        Stat::new(4, 18),
                        Stat::new(5, 22),
                        Stat::new(6, 24),
                        Stat::new(7, 31),
                        Stat::new(8, 33),
                        Stat::new(10, 46),
                        Stat::new(1, 91),
                        Stat::new(9, 106),]);
    }
}
