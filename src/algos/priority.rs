use crate::{Process, SchedulerResult};

pub fn highest_priority_first<'a, I>(processes: I) -> SchedulerResult
where
    I: IntoIterator<Item = &'a mut Process>,
{
    let mut process_vec = processes.into_iter().collect::<Vec<&mut Process>>();
    let mut tick = 0;
    while !process_vec.iter().all(|p| p.is_finished()) {
        if let Some(process_to_run) = process_vec
            .iter_mut()
            .filter(|p| !p.is_finished() && (p.arrival_time <= tick))
            .min_by_key(|p| p.priority)
        {
            process_to_run.run_to_completion();
            tick += process_to_run.burst_time;
            process_to_run.exit_time = Some(tick);
        } else {
            tick += 1;
            continue;
        }
    }
    Process::compute_result(process_vec)
}

#[test]
fn priority_test() {
    use super::priority::*;
    let mut processes = vec![
        Process::new(0, 0, 10, 2),
        Process::new(1, 1, 6, 5),
        Process::new(2, 3, 2, 3),
        Process::new(3, 5, 4, 1),
    ];

    let result = highest_priority_first(processes.iter_mut());
    assert_eq!(result.total_wait_time, 31);
    assert_eq!(result.average_wait_time, 7.75);
    assert_eq!(result.total_turnaround_time, 53);
    assert_eq!(result.average_turnaround_time, 13.25);
}
