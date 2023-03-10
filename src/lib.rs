pub mod algos;

#[derive(Default, Debug)]
pub struct SchedulerResult {
    pub total_wait_time: usize,
    pub average_wait_time: f64,
    pub total_turnaround_time: usize,
    pub average_turnaround_time: f64,
}

#[derive(Debug, Clone)]
pub enum ProcessState {
    NotInSytstem,
    Ready,
    Finished,
}

impl Default for ProcessState {
    fn default() -> Self {
        ProcessState::NotInSytstem
    }
}

#[derive(Default, Debug, Clone)]
pub struct Process {
    pub pid: usize,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub exit_time: Option<usize>,
    pub priority: usize,
    pub state: ProcessState,
    progress: usize,
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Process {
    pub fn new(pid: usize, arrival_time: usize, burst_time: usize, priority: usize) -> Self {
        Process {
            pid,
            arrival_time,
            priority,
            burst_time,
            exit_time: None,
            progress: 0,
            state: ProcessState::NotInSytstem,
        }
    }

    pub fn compute_result<'a, I>(processes: I) -> SchedulerResult
    where
        I: IntoIterator<Item = &'a mut Process>,
    {
        let mut total_turnaround_time = 0;
        let mut total_wait_time = 0;

        let mut len = 0;
        for process in processes.into_iter() {
            len += 1;
            total_turnaround_time += process.turnaround_time().unwrap();
            total_wait_time += process.wait_time().unwrap();
        }

        SchedulerResult {
            total_wait_time,
            average_wait_time: total_wait_time as f64 / len as f64,
            total_turnaround_time,
            average_turnaround_time: total_turnaround_time as f64 / len as f64,
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.state, ProcessState::Finished)
    }

    pub fn is_insystem(&self) -> bool {
        !matches!(self.state, ProcessState::NotInSytstem)
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, ProcessState::Ready)
    }

    pub fn turnaround_time(&self) -> Option<usize> {
        Some(self.exit_time? - self.arrival_time)
    }

    pub fn wait_time(&self) -> Option<usize> {
        Some(self.turnaround_time()? - self.burst_time)
    }

    pub fn time_to_complete(&self) -> usize {
        self.burst_time - self.progress
    }

    pub fn run_for(&mut self, time: usize) -> usize {
        self.progress += time;

        if self.progress >= self.burst_time {
            self.state = ProcessState::Finished;
            self.progress = self.burst_time;
        }

        self.progress
    }

    pub fn run_once(&mut self) -> usize {
        self.run_for(1)
    }

    pub fn run_to_completion(&mut self) -> usize {
        self.run_for(self.burst_time)
    }
}
