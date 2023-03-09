use once_cell::sync::Lazy;
use scheduler::{algos::*, Process};

static PROCESSES: Lazy<Vec<Process>> = Lazy::new(|| {
    vec![
        Process::new(0, 0, 10, 2),
        Process::new(1, 1, 6, 5),
        Process::new(2, 3, 2, 3),
        Process::new(3, 5, 4, 1),
    ]
});

#[test]
fn fcfs() {
    use fcfs::first_come_first_serve;

    let result = first_come_first_serve(PROCESSES.clone().iter_mut());
    assert_eq!(result.total_wait_time, 35);
    assert_eq!(result.average_wait_time, 8.75);
    assert_eq!(result.total_turnaround_time, 57);
    assert_eq!(result.average_turnaround_time, 14.25);
}

#[test]
fn sjf() {
    use sjf::shortest_job_first;

    let result = shortest_job_first(PROCESSES.clone().iter_mut());
    assert_eq!(result.total_wait_time, 29);
    assert_eq!(result.average_wait_time, 7.25);
    assert_eq!(result.total_turnaround_time, 51);
    assert_eq!(result.average_turnaround_time, 12.75);
}

#[test]
fn srtf() {
    use srtf::shortest_remaining_time_first;

    let result = shortest_remaining_time_first(PROCESSES.clone().iter_mut());
    assert_eq!(result.total_wait_time, 18);
    assert_eq!(result.average_wait_time, 4.5);
    assert_eq!(result.total_turnaround_time, 40);
    assert_eq!(result.average_turnaround_time, 10.0);
}

#[test]
fn priority() {
    use priority::highest_priority_first;

    let result = highest_priority_first(PROCESSES.clone().iter_mut());
    assert_eq!(result.total_wait_time, 31);
    assert_eq!(result.average_wait_time, 7.75);
    assert_eq!(result.total_turnaround_time, 53);
    assert_eq!(result.average_turnaround_time, 13.25);
}

#[test]
fn priority_preemptive() {
    use priority_preemptive::highest_priority_first_preemptive;

    let result = highest_priority_first_preemptive(PROCESSES.clone().iter_mut());
    assert_eq!(result.total_wait_time, 30);
    assert_eq!(result.average_wait_time, 7.5);
    assert_eq!(result.total_turnaround_time, 52);
    assert_eq!(result.average_turnaround_time, 13.0);
}

#[test]
fn round_robin() {
    use round_robin::round_robin;

    let result = round_robin(PROCESSES.clone().iter_mut(), 2);
    assert_eq!(result.total_wait_time, 33);
    assert_eq!(result.average_wait_time, 8.25);
    assert_eq!(result.total_turnaround_time, 55);
    assert_eq!(result.average_turnaround_time, 13.75);
}
