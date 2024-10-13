We're going to create a lightweight scheduling algorithm/simulator.  Scheduling algorithms are used by operating systems to determine which processes get time on the cpu and when.  Recently people have written schedulers for the Linux OS in rust.  There's a famous one that was written the optimize game performance.

Each process has an id, a runtime remaining, and a time slice

We'll stick all of the processes in the queue and then simulate executing them as so:

- set simulated_time to zero
- while the queue is not empty
  - dequeue a process
  - get the minimum of that process's runtime_remaining or time_slice let's call it run_time
  - add run_time to simulated time
  - subtract run_time from that process's runtime_remaining
  - if that process's runtime_remaining is greater than zero, enqueue the process at the end
  
repeat the loop until the queue is empty

while debugging it might be helpful to output what process you dequeued, what run_time was, and whether the process finished or was preempted and put back on the queue

Let's keep track of when(in simulated_time) a process completes and we can use that to write some test cases

Test Cases:

1. 
  - Processes:
    - | id | remaining | slice |
      | :-- | :-- | :-- |
      | 1 | 23 | 5 |
      | 2 | 29 | 7 |
  - Expected Results:
    - | id | completion time |
      | :-- | :-- |
      | 1 | 51 |
      | 2 | 52 |
2. 
  - Processes:
    - | id | remaining | slice |
      | :--: | :--: | :--: |
      | 1 | 9 | 9 |
      | 2 | 7 | 7 |
      | 3 | 6 | 7 |
      | 4 | 1 | 10 |
      | 5 | 8 | 8 |
      | 6 | 45 | 9 |
      | 7 | 53 | 10 |
      | 8 | 11 | 10 |
      | 9 | 9 | 10 |
      | 10 | 2 | 6 |
  - Expected Results:
    - | id | completion time |
      | :--: | :--: |
      | 1 | 9 |
      | 2 | 16 |
      | 3 | 22 |
      | 4 | 23 |
      | 5 | 31 |
      | 9 | 69 |
      | 10 | 71 |
      | 8 | 91 |
      | 6 | 138 |
      | 7 | 151 |
3. 
  - Processes:
    - | id | remaining | slice |
      | :-- | :-- | :-- |
      | 1 | 6 | 11 |
      | 2 | 3 | 5 |
      | 3 | 18 | 6 |
      | 4 | 5 | 5 |
      | 5 | 3 | 7 |
      | 6 | 14 | 7 |
      | 7 | 3 | 3 |
      | 8 | 6 | 10 |
      | 9 | 29 | 5 |
      | 10 | 7 | 7 |
  - Expected Results:
    - | id | completion time |
      | :-- | :-- |
      | 1 | 6 |
      | 2 | 9 |
      | 4 | 20 |
      | 5 | 23 |
      | 7 | 33 |
      | 8 | 39 |
      | 10 | 51 |
      | 6 | 64 |
      | 3 | 75 |
      | 9 | 94 |
4. 
  - Processes:
    - | id | remaining | slice |
      | :-- | :-- | :-- |
      | 1 | 15 | 3 |
      | 2 | 3 | 10 |
      | 3 | 6 | 6 |
      | 4 | 6 | 6 |
      | 5 | 4 | 4 |
      | 6 | 2 | 9 |
      | 7 | 7 | 7 |
      | 8 | 2 | 3 |
      | 9 | 59 | 11 |
      | 10 | 2 | 8 |
  - Expected Results:
    - | id | completion time |
      | :-- | :-- |
      | 2 | 6 |
      | 3 | 12 |
      | 4 | 18 |
      | 5 | 22 |
      | 6 | 24 |
      | 7 | 31 |
      | 8 | 33 |
      | 10 | 46 |
      | 1 | 91 |
      | 9 | 106 |
