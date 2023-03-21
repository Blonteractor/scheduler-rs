pub mod priority;
pub mod round_robin;
pub mod srtf;

#[macro_export]
macro_rules! run_preemptive {
    ($p:ident, $g:ident, $t:ident) => {
        let mut node = GranttNode::default();
        node.pid = $p.pid;
        node.start = $t;

        $p.run_once();
        $t += 1;

        node.end = $t;
        $g.push_back(node);

    };

    ($p:ident, $g:ident, $t:ident, $q:expr) => {
        let mut node = GranttNode::default();
        node.pid = $p.pid;
        node.start = $t;

        $p.run_for($q);
        $t += $q;

        node.end = $t;
        $g.push_back(node);

    };
}
