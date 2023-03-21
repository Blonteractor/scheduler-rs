pub mod fcfs;
pub mod priority;
pub mod sjf;

#[macro_export]
macro_rules! run {
    ($p:ident, $g:ident, $t:ident) => {
        let mut node = GranttNode::default();
        node.pid = $p.pid;
        node.start = $t;

        $p.run_to_completion();
        $t += $p.burst_time;

        node.end = $t;
        $p.exit_time = Some($t);
        $g.push_back(node);
    };
}
