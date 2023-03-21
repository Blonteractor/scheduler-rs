use std::collections::VecDeque;

use crate::{Process, ProcessState, SchedulerResult, GranttNode, run_preemptive};

pub fn round_robin<'a, I>(processes: I, quantum: usize) -> SchedulerResult
where
    I: IntoIterator<Item = &'a mut Process>,
{
    let mut process_vec = processes.into_iter().collect::<Vec<&mut Process>>();
    let mut tick = 0;
    let mut last_unfinished_process_id: Option<usize> = None;
    let mut waiting_queue: VecDeque<usize> = VecDeque::with_capacity(process_vec.len());
    let mut process_to_run;
    let mut grantt_chart = VecDeque::with_capacity(process_vec.len());

    while !process_vec.iter().all(|p| p.is_finished()) {
        waiting_queue.extend(
            process_vec
                .iter_mut()
                .filter(|p| {
                    p.arrival_time <= tick
                        && !p.is_insystem()
                        && last_unfinished_process_id.unwrap_or(usize::MAX) != p.pid
                })
                .map(|p| {
                    p.state = ProcessState::Ready;
                    p.pid
                }),
        );

        if let Some(lup) = last_unfinished_process_id {
            waiting_queue.push_back(lup);
            last_unfinished_process_id = None;
        }

        process_to_run = &mut process_vec[waiting_queue.pop_front().unwrap()];
        process_to_run.state = ProcessState::NotInSytstem;
        run_preemptive!(process_to_run, grantt_chart, tick, quantum);

        if process_to_run.is_finished() {
            process_to_run.exit_time = Some(tick);
        } else {
            process_to_run.state = ProcessState::Ready;
            last_unfinished_process_id = Some(process_to_run.pid);
        }
    }

    Process::compute_result(process_vec, grantt_chart, false)
}
