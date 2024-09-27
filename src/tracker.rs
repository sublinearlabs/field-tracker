use std::{cell::RefCell, fmt::Display};

const GLOBAL_SUMMARY: &'static str = "Global Summary";

thread_local! {
    static GLOBAL_TRACKER: RefCell<Tracker> = RefCell::new(Tracker::new());
}

#[derive(Debug, Clone)]
pub struct ReportValues {
    add: i32,
    mul: i32,
    inv: i32,
}

#[derive(Debug, Clone)]
pub struct Report {
    name: &'static str,
    values: ReportValues,
    children: Option<Vec<Report>>,
}

impl ReportValues {
    pub fn new() -> Self {
        Self {
            add: 0,
            mul: 0,
            inv: 0,
        }
    }
}

impl Report {
    pub fn new(name: &'static str) -> Self {
        Report {
            name,
            values: ReportValues::new(),
            children: None,
        }
    }

    pub fn merge(&mut self, child_report: Report) {
        self.values.add += child_report.values.add;
        self.values.mul += child_report.values.mul;
        self.values.inv += child_report.values.inv;
        match &mut self.children {
            Some(children) => {
                children.push(child_report);
            }
            None => self.children = Some(vec![child_report]),
        }
    }
}

#[derive(Debug)]
pub struct Tracker {
    stack: Vec<Report>,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker {
            stack: vec![Report::new(GLOBAL_SUMMARY)],
        }
    }

    pub fn start(name: &'static str) {
        GLOBAL_TRACKER.with(|v| v.borrow_mut().stack.push(Report::new(name)));
    }

    pub fn end() {
        GLOBAL_TRACKER.with(|v| {
            let stack = &mut v.borrow_mut().stack;
            if stack.len() <= 1 {
                panic!("Tracking not Started");
            }
            let current_active = stack.pop().unwrap();
            stack.last_mut().unwrap().merge(current_active)
        });
    }

    pub fn summary() -> Report {
        GLOBAL_TRACKER.with(|tracker| {
            // TODO: investigate how expensive this is
            let mut stack_copy = tracker.borrow().stack.clone();

            while stack_copy.len() >= 2 {
                let child = stack_copy
                    .pop()
                    .expect("confirmed stack copy has at least 2 elements");
                stack_copy.last_mut().unwrap().merge(child);
            }

            stack_copy.pop().unwrap()
        })
    }

    pub fn reset() {
        GLOBAL_TRACKER.with(|v| v.replace(Tracker::new()));
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub fn update_add() {
    GLOBAL_TRACKER.with(|v| v.borrow_mut().stack.last_mut().unwrap().values.add += 1);
}
pub fn update_mul() {
    GLOBAL_TRACKER.with(|v| v.borrow_mut().stack.last_mut().unwrap().values.mul += 1);
}
pub fn update_inv() {
    GLOBAL_TRACKER.with(|v| v.borrow_mut().stack.last_mut().unwrap().values.inv += 1);
}

#[cfg(test)]
pub mod tests {
    use super::{update_add, update_inv, update_mul, Tracker};

    #[test]
    pub fn test_nested_tracker_summary_call() {
        Tracker::reset();
        Tracker::start("GKR");
        update_inv();
        assert_eq!(
            Tracker::summary().values.inv,
            1,
            "Wrong summary for inverse"
        );
        Tracker::start("Sumcheck");
        update_add();
        update_add();
        assert_eq!(Tracker::summary().values.add, 2, "Wrong summary for add");
        update_mul();
        update_inv();
        assert_eq!(Tracker::summary().values.add, 2, "Wrong summary for add");
        assert_eq!(Tracker::summary().values.inv, 2, "Wrong summary for inv");
        assert_eq!(Tracker::summary().values.mul, 1, "Wrong summary for mul");
        Tracker::end();
        Tracker::start("Sumcheck");
        update_mul();
        update_mul();
        update_mul();
        assert_eq!(Tracker::summary().values.mul, 4, "Wrong summary for mul");
        Tracker::end();
        Tracker::end();
        println!("{}", Tracker::summary());
        Tracker::reset();
        Tracker::summary();
        assert_eq!(Tracker::summary().values.add, 0, "Wrong summary for add");
        assert_eq!(Tracker::summary().values.inv, 0, "Wrong summary for inv");
        assert_eq!(Tracker::summary().values.mul, 0, "Wrong summary for mul");
    }

    #[test]
    #[should_panic]
    pub fn test_end_tracker_inappropriately() {
        Tracker::reset();
        Tracker::summary();
        Tracker::start("GKR");
        Tracker::summary();
        Tracker::start("Sumcheck");
        Tracker::summary();
        Tracker::end();
        Tracker::summary();
        Tracker::end();
        Tracker::summary();
        Tracker::end();
        Tracker::reset();
    }

    #[test]
    pub fn test_one_layered_summary() {
        Tracker::start("GKR");
        update_add();
        update_add();
        update_inv();
        update_mul();
        Tracker::end();
        assert_eq!(Tracker::summary().values.add, 2, "Wrong number of add op");
        assert_eq!(Tracker::summary().values.mul, 1, "Wrong number of mul op");
        assert_eq!(Tracker::summary().values.inv, 1, "Wrong number of inv op");
    }
}
