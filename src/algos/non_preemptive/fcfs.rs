use std::collections::VecDeque;

use crate::{run, GranttNode, Process, SchedulerResult};

pub fn first_come_first_serve<'a, I>(processes: I) -> SchedulerResult
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
            .min_by_key(|p| p.arrival_time)
        {
            run!(process_to_run, grantt_chart, tick);
        } else {
            tick += 1;
            continue;
        }
    }
    Process::compute_result(process_vec, grantt_chart, true)
}
