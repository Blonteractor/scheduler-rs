use crate::{Process, SchedulerResult};

pub fn first_come_first_serve<'a, I>(processes: I) -> SchedulerResult
where
    I: IntoIterator<Item = &'a mut Process>,
{
    let mut process_vec = processes.into_iter().collect::<Vec<&mut Process>>();
    let mut tick = 0;
    while !process_vec.iter().all(|p| p.is_finished()) {
        if let Some(process_to_run) = process_vec
            .iter_mut()
            .filter(|p| !p.is_finished() && (p.arrival_time <= tick))
            .min_by_key(|p| p.arrival_time)
        {
            process_to_run.run_to_completion();
            tick += process_to_run.burst_time;
            process_to_run.exit_time = Some(tick);
            println!("{:#?}", tick);
        } else {
            tick += 1;
            continue;
        }
    }
    Process::compute_result(process_vec)
}

#[test]
fn fcfs_test() {
    use super::fcfs::*;
    let mut processes = vec![
        Process::new(0, 10, 0),
        Process::new(1, 6, 0),
        Process::new(3, 2, 0),
        Process::new(5, 4, 0),
    ];

    let result = first_come_first_serve(processes.iter_mut());
    assert_eq!(result.total_wait_time, 35);
    assert_eq!(result.average_wait_time, 8.75);
    assert_eq!(result.total_turnaround_time, 57);
    assert_eq!(result.average_turnaround_time, 14.25);
}
