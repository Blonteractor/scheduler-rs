use crate::{Process, SchedulerResult};

pub fn shortest_job_first<'a, I>(processes: I) -> SchedulerResult
where
    I: IntoIterator<Item = &'a mut Process>,
{
    let mut process_vec = processes.into_iter().collect::<Vec<&mut Process>>();
    let mut tick = 0;
    while !process_vec.iter().all(|p| p.is_finished()) {
        if let Some(process_to_run) = process_vec
            .iter_mut()
            .filter(|p| !p.is_finished() && (p.arrival_time <= tick))
            .min_by_key(|p| p.burst_time)
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
fn sjf_test() {
    use super::sjf::*;
    let mut processes = vec![
        Process::new(0, 0, 10, 0),
        Process::new(1, 1, 6, 0),
        Process::new(2, 3, 2, 0),
        Process::new(3, 5, 4, 0),
    ];

    let result = shortest_job_first(processes.iter_mut());
    assert_eq!(result.total_wait_time, 29);
    assert_eq!(result.average_wait_time, 7.25);
    assert_eq!(result.total_turnaround_time, 51);
    assert_eq!(result.average_turnaround_time, 12.75);
}
