use std::fmt::Display;

static mut GLOBAL_TRACKER: Tracker = Tracker { stack: vec![] };

#[derive(Debug, Clone)]
pub struct ReportValues {
    add: i32,
    mul: i32,
    inv: i32,
}

#[derive(Debug, Clone)]
pub struct Report {
    name: String,
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
    pub fn new(name: String) -> Self {
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
            stack: vec![Report::new("Global Summary".to_string())],
        }
    }

    pub fn start(name: String) {
        unsafe { GLOBAL_TRACKER.stack.push(Report::new(name)) };
    }

    pub fn end() {
        let stack = unsafe { &mut GLOBAL_TRACKER.stack };
        if stack.len() <= 1 {
            panic!("Tracking not Started");
        }
        let current_active = stack.pop().unwrap();
        stack.last_mut().unwrap().merge(current_active);
    }

    pub fn summary() -> Report {
        let stack = unsafe { &GLOBAL_TRACKER.stack };
        stack.iter().skip(1).fold(
            stack
                .first()
                .unwrap_or(&Report::new("Global Summary".to_string()))
                .clone(),
            |mut init, report| {
                init.merge(report.clone());
                init
            },
        )
    }

    pub fn reset() {
        unsafe { GLOBAL_TRACKER.stack = vec![Report::new("Global Summary".to_string())] };
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!(
            "
            {}\n
            \t Add ops: {}, Mul ops: {}, Inv ops: {} \n
            ",
            self.name, self.values.add, self.values.mul, self.values.inv
        ));

        match &self.children {
            Some(children) => {
                children.iter().for_each(|child| {
                    let _ = f.write_fmt(format_args!(
                        "
                        {}\n
                        \t Add ops: {}, Mul ops: {}, Inv ops: {} \n
                        ",
                        child.name, child.values.add, child.values.mul, child.values.inv
                    ));
                });
            }
            None => {}
        }

        Ok(())
    }
}

pub fn update_add() {
    let stack = unsafe { &mut GLOBAL_TRACKER.stack };
    stack.last_mut().unwrap().values.add += 1;
}
pub fn update_mul() {
    let stack = unsafe { &mut GLOBAL_TRACKER.stack };
    stack.last_mut().unwrap().values.mul += 1;
}
pub fn update_inv() {
    let stack = unsafe { &mut GLOBAL_TRACKER.stack };
    stack.last_mut().unwrap().values.inv += 1;
}

#[cfg(test)]
pub mod tests {
    use super::{update_add, update_inv, update_mul, Tracker};

    #[test]
    pub fn test_nested_tracker_summary_call() {
        Tracker::reset();
        Tracker::start("GKR".to_string());
        update_inv();
        assert_eq!(
            Tracker::summary().values.inv,
            1,
            "Wrong summary for inverse"
        );
        Tracker::start("Sumcheck".to_string());
        update_add();
        update_add();
        assert_eq!(Tracker::summary().values.add, 2, "Wrong summary for add");
        update_mul();
        update_inv();
        assert_eq!(Tracker::summary().values.add, 2, "Wrong summary for add");
        assert_eq!(Tracker::summary().values.inv, 2, "Wrong summary for inv");
        assert_eq!(Tracker::summary().values.mul, 1, "Wrong summary for mul");
        Tracker::end();
        Tracker::start("Sumcheck".to_string());
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
        Tracker::start("GKR".to_string());
        Tracker::summary();
        Tracker::start("Sumcheck".to_string());
        Tracker::summary();
        Tracker::end();
        Tracker::summary();
        Tracker::end();
        Tracker::summary();
        Tracker::end();
        Tracker::reset();
    }
}
