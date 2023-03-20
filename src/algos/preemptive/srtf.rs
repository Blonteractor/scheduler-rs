use std::collections::VecDeque;

use crate::{Process, SchedulerResult, GranttNode};

pub fn shortest_remaining_time_first<'a, I>(processes: I) -> SchedulerResult
where
    I: IntoIterator<Item = &'a mut Process>,
{
    let mut process_vec = processes.into_iter().collect::<Vec<&mut Process>>();
    let mut tick = 0;
    let mut grantt_chart = VecDeque::with_capacity(process_vec.len());

    while !process_vec.iter().all(|p| p.is_finished()) {
        if let Some(process_to_run) = process_vec
            .iter_mut()
            .filter(|p| !p.is_finished() && (p.arrival_time <= tick))
            .min_by_key(|p| p.time_to_complete())
        {
            let mut node = GranttNode::default();
            node.pid = process_to_run.pid;
            node.start = tick;

            process_to_run.run_once();
            tick += 1;
            node.end = tick;
            grantt_chart.push_back(node);
            if process_to_run.is_finished() {
                process_to_run.exit_time = Some(tick);
            }
        } else {
            tick += 1;
            continue;
        }
    }
    Process::compute_result(process_vec, grantt_chart, true)
}
