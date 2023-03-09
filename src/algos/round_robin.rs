use std::collections::VecDeque;

use crate::{Process, ProcessState, SchedulerResult};

pub fn round_robin<I>(processes: I, quantum: usize) -> SchedulerResult
where
    I: IntoIterator<Item = Process>,
{
    let mut process_vec = processes.into_iter().collect::<Vec<Process>>();
    let mut tick = 0;
    let mut last_unfinished_process_id: Option<usize> = None;
    let mut waiting_queue: VecDeque<usize> = VecDeque::with_capacity(process_vec.len());
    let mut process_to_run;

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
        process_to_run.run_for(quantum);
        tick += quantum;

        if process_to_run.is_finished() {
            process_to_run.exit_time = Some(tick);
        } else {
            process_to_run.state = ProcessState::Ready;
            last_unfinished_process_id = Some(process_to_run.pid);
        }
    }

    Process::compute_result(&mut process_vec)
}

// pub fn round_robin<I>(processes: I, quantum: usize) -> SchedulerResult
// where
//     I: IntoIterator<Item = Process>,
// {
//     let mut process_vec = processes.into_iter().collect::<Vec<Process>>();
//     let mut tick = 0;
//     let mut last_unfinished_process: Option<RefCell<Process>> = None;
//     let mut waiting_queue: VecDeque<RefCell<Process>> = VecDeque::with_capacity(process_vec.len());
//     let mut process_to_run;
//     while !process_vec.iter().all(|p| p.is_finished()) {
//         waiting_queue.extend(
//             process_vec
//                 .iter()
//                 .filter(|p| -> bool {
//                     !p.is_finished() && p.arrival_time <= tick && {
//                         last_unfinished_process.is_some()
//                             && last_unfinished_process.as_ref().unwrap().pid != p.pid
//                     }
//                 })
//                 .map(|p| Rc::new(p)),
//         );

//         if let Some(lup) = last_unfinished_process {
//             waiting_queue.push_back(lup.clone());
//         }

//         process_to_run = waiting_queue.pop_front().unwrap();
//         process_to_run.run_for(quantum);
//         process_to_run.state = ProcessState::NotInSytstem;
//         tick += quantum;

//         if process_to_run.is_finished() {
//             process_to_run.exit_time = Some(tick);
//         } else {
//             process_to_run.state = ProcessState::Ready;
//             last_unfinished_process = Some(process_to_run);
//         }
//     }
//     Process::compute_result(&mut process_vec)
// }

#[test]
fn round_robin_test() {
    use super::round_robin::*;
    let processes = vec![
        Process::new(0, 0, 10, 2),
        Process::new(1, 1, 6, 5),
        Process::new(2, 3, 2, 3),
        Process::new(3, 5, 4, 1),
    ];

    let result = round_robin(processes, 2);
    assert_eq!(result.total_wait_time, 33);
    assert_eq!(result.average_wait_time, 8.25);
    assert_eq!(result.total_turnaround_time, 55);
    assert_eq!(result.average_turnaround_time, 13.75);
}
