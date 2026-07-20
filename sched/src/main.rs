use my_sched::add;
use my_sched::executor::run_default;
use my_sched::executor::task_local::run_task;
use my_sched::executor::worker::do_work;

fn main() {
    let ret = add(1, 2);

    run_default();
    run_task();
    do_work();
    println!("Hello, world!, my add: {ret}");
}
