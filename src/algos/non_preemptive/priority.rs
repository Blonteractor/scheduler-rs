use std::collections::VecDeque;

use crate::{run, Process, SchedulerResult, GranttNode};

pub fn highest_priority_first<'a, I>(processes: I) -> SchedulerResult
where
    I: IntoIterator<Item = &'a mut Process>,
{
    let mut process_vec = processes.into_iter().collect::<Vec<&mut Process>>();
    let mut tick = 0;
    let mut grantt_chart: VecDeque<GranttNode> = VecDeque::with_capacity(process_vec.len());
    while !process_vec.iter().all(|p| p.is_finished()) {
        if let Some(process_to_run) = process_vec
            .iter_mut()
            .filter(|p| !p.is_finished() && (p.arrival_time <= tick))
            .min_by_key(|p| p.priority)
        {
            run!(process_to_run, grantt_chart, tick);
        } else {
            tick += 1;
            continue;
        }
    }
    Process::compute_result(process_vec, grantt_chart, true)
}
