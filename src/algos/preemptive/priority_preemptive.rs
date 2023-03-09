use crate::{Process, SchedulerResult};

pub fn highest_priority_first_preemptive<'a, I>(processes: I) -> SchedulerResult
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
            process_to_run.run_once();
            tick += 1;
            if process_to_run.is_finished() {
                process_to_run.exit_time = Some(tick);
            }
        } else {
            tick += 1;
            continue;
        }
    }
    Process::compute_result(process_vec)
}