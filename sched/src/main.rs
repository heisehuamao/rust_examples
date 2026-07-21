use my_sched::add;
use my_sched::executor::{Executor, run_default};
// use my_sched::executor::task_local::run_task;
// use my_sched::executor::worker::do_work;

fn main() {
    let ret = add(1, 2);

    run_default();
    // run_task();
    // do_work();
    println!("Hello, world!, my add: {ret}");

    let mut executor = Executor::default();
    // Spawn some local tasks
    let id1 = executor.spawn("task_1", async {
        println!("[Task 1] Step 1");
        // Yield execution back to executor
        // tokio::task::yield_now().await;
        println!("[Task 1] Step 2");
    });

    let id2 = executor.spawn("task_2", async {
        println!("[Task 2] Completed immediately");
    });

    println!("task 1: {id1}, task 2: {id2}");

    // println!("Initial queue length: {}", manager.queue_len());

    // Initialize and run the executor
    executor.run();

    // println!(
    //     "All tasks finished. Active tasks left: {}",
    //     executor.manager_mut().active_task_count()
    // );
}
