use num_format::{Locale, ToFormattedString};
use std::{cell::RefCell, fmt::Display};
use treeline::Tree;

const GLOBAL_SUMMARY: &str = "Global Summary";

thread_local! {
    static GLOBAL_TRACKER: RefCell<Tracker> = RefCell::new(Tracker::new());
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReportValues {
    pub add: usize,
    pub sub: usize,
    pub mul: usize,
    pub inv: usize,
}

impl Display for ReportValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "add: {}, sub: {}, mul: {}, inv: {}",
            self.add.to_formatted_string(&Locale::en),
            self.sub.to_formatted_string(&Locale::en),
            self.mul.to_formatted_string(&Locale::en),
            self.inv.to_formatted_string(&Locale::en)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    name: &'static str,
    pub values: ReportValues,
    pub children: Option<Vec<Report>>,
}

impl Report {
    fn new(name: &'static str) -> Self {
        Report {
            name,
            values: ReportValues::default(),
            children: None,
        }
    }

    fn merge(&mut self, child_report: Report) {
        self.values.add += child_report.values.add;
        self.values.sub += child_report.values.sub;
        self.values.mul += child_report.values.mul;
        self.values.inv += child_report.values.inv;
        match &mut self.children {
            Some(children) => {
                children.push(child_report);
            }
            None => self.children = Some(vec![child_report]),
        }
    }

    #[allow(dead_code)]
    fn to_string(&self, tab_count: usize) -> String {
        let mut output = String::new();

        let tab_str = "  ".repeat(tab_count);

        output.push_str(format!("\n{}{}\n", tab_str, self.name).as_str());
        output.push_str(format!(" {}{}\n", tab_str, self.values).as_str());

        match &self.children {
            None => {}
            Some(children) => {
                for child in children {
                    output.push_str(child.to_string(tab_count + 1).as_str())
                }
            }
        }

        output
    }

    fn build_tree(&self) -> Tree<String> {
        let mut res = Tree::root(self.name.to_string());

        res.push(Tree::root(self.values.to_string()));

        match &self.children {
            None => {}
            Some(children) => {
                for child in children {
                    res.push(child.build_tree());
                }
            }
        }

        res
    }
}

#[derive(Debug)]
pub struct Tracker {
    stack: Vec<Report>,
}

impl Tracker {
    fn new() -> Self {
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

    #[allow(dead_code)]
    fn reset() {
        GLOBAL_TRACKER.with(|v| v.replace(Tracker::new()));
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.build_tree()).as_str())
    }
}

pub fn update_add() {
    GLOBAL_TRACKER.with(|v| v.borrow_mut().stack.last_mut().unwrap().values.add += 1);
}

pub fn update_sub() {
    GLOBAL_TRACKER.with(|v| v.borrow_mut().stack.last_mut().unwrap().values.sub += 1);
}
pub fn update_mul() {
    GLOBAL_TRACKER.with(|v| v.borrow_mut().stack.last_mut().unwrap().values.mul += 1);
}
pub fn update_inv() {
    GLOBAL_TRACKER.with(|v| v.borrow_mut().stack.last_mut().unwrap().values.inv += 1);
}

#[cfg(test)]
mod tests {
    use super::{update_add, update_inv, update_mul, Tracker};

    fn gkr_sumcheck_squence() {
        Tracker::start("gkr");
        {
            Tracker::start("sumcheck");
            {
                update_add();
                update_mul();
                Tracker::start("poly");
                Tracker::end();
            }
            Tracker::end();

            Tracker::start("sumcheck");
            {
                update_inv();
                Tracker::start("poly");
                update_mul();
                Tracker::end();
            }
            Tracker::end();
        }
        update_add();
        Tracker::end();
    }

    #[test]
    fn test_nested_tracker_summary_call() {
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

        Tracker::reset();
        Tracker::summary();
        assert_eq!(Tracker::summary().values.add, 0, "Wrong summary for add");
        assert_eq!(Tracker::summary().values.inv, 0, "Wrong summary for inv");
        assert_eq!(Tracker::summary().values.mul, 0, "Wrong summary for mul");
    }

    #[test]
    #[should_panic]
    fn test_end_tracker_inappropriately() {
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
    fn test_one_layered_summary() {
        Tracker::reset();
        Tracker::start("GKR");
        update_add();
        update_add();
        update_inv();
        update_mul();
        Tracker::end();
        assert_eq!(Tracker::summary().values.add, 2, "Wrong number of add op");
        assert_eq!(Tracker::summary().values.mul, 1, "Wrong number of mul op");
        assert_eq!(Tracker::summary().values.inv, 1, "Wrong number of inv op");
        Tracker::reset();
    }

    #[test]
    #[should_panic]
    fn test_should_panic_on_global_pop() {
        Tracker::reset();
        Tracker::end();
        Tracker::reset();
    }

    #[test]
    fn test_correct_summary_report() {
        // TODO: figure out a better way to test summary history change
        //  new way should be robust to new tracking items e.g div, exp ...

        Tracker::reset();
        Tracker::start("gkr");
        {
            update_add();
            let summary_1 = Tracker::summary();
            assert_eq!(&summary_1.values.add, &1);
            assert_eq!(&summary_1.children.as_ref().unwrap()[0].values.add, &1);

            Tracker::start("sumcheck");
            let summary_2 = {
                update_add();
                update_inv();
                let summary_2 = Tracker::summary();

                // verify add
                assert_eq!(&summary_2.values.add, &2);
                assert_eq!(&summary_2.children.as_ref().unwrap()[0].values.add, &2);
                assert_eq!(
                    &summary_2.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[0]
                        .values
                        .add,
                    &1
                );

                // verify mul
                assert_eq!(&summary_2.values.mul, &0);
                assert_eq!(&summary_2.children.as_ref().unwrap()[0].values.mul, &0);
                assert_eq!(
                    &summary_2.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[0]
                        .values
                        .mul,
                    &0
                );

                // verify inv
                assert_eq!(&summary_2.values.inv, &1);
                assert_eq!(&summary_2.children.as_ref().unwrap()[0].values.inv, &1);
                assert_eq!(
                    &summary_2.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[0]
                        .values
                        .inv,
                    &1
                );

                summary_2
            };
            Tracker::end();

            // summary_2 == summary_3
            let summary_3 = Tracker::summary();
            assert_eq!(summary_3, summary_2);

            update_mul();
            let summary_4 = Tracker::summary();

            // verify add
            assert_eq!(&summary_4.values.add, &2);
            assert_eq!(&summary_4.children.as_ref().unwrap()[0].values.add, &2);
            assert_eq!(
                &summary_4.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[0]
                    .values
                    .add,
                &1
            );

            // verify mul
            assert_eq!(&summary_4.values.mul, &1);
            assert_eq!(&summary_4.children.as_ref().unwrap()[0].values.mul, &1);
            assert_eq!(
                &summary_4.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[0]
                    .values
                    .mul,
                &0
            );

            // verify inv
            assert_eq!(&summary_4.values.inv, &1);
            assert_eq!(&summary_4.children.as_ref().unwrap()[0].values.inv, &1);
            assert_eq!(
                &summary_4.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[0]
                    .values
                    .inv,
                &1
            );

            Tracker::start("sumcheck");
            {
                update_mul();
                update_mul();
                let summary_5 = Tracker::summary();

                // verify add
                assert_eq!(&summary_5.values.add, &2);
                assert_eq!(&summary_5.children.as_ref().unwrap()[0].values.add, &2);
                assert_eq!(
                    &summary_5.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[0]
                        .values
                        .add,
                    &1
                );
                assert_eq!(
                    &summary_5.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[1]
                        .values
                        .add,
                    &0
                );

                // verify mul
                assert_eq!(&summary_5.values.mul, &3);
                assert_eq!(&summary_5.children.as_ref().unwrap()[0].values.mul, &3);
                assert_eq!(
                    &summary_5.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[0]
                        .values
                        .mul,
                    &0
                );
                assert_eq!(
                    &summary_5.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[1]
                        .values
                        .mul,
                    &2
                );

                // verify inv
                assert_eq!(&summary_5.values.inv, &1);
                assert_eq!(&summary_5.children.as_ref().unwrap()[0].values.inv, &1);
                assert_eq!(
                    &summary_5.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[0]
                        .values
                        .inv,
                    &1
                );
                assert_eq!(
                    &summary_5.children.as_ref().unwrap()[0]
                        .children
                        .as_ref()
                        .unwrap()[1]
                        .values
                        .inv,
                    &0
                );

                update_add();
            }
            Tracker::end();
            let summary_6 = Tracker::summary();

            // verify add
            assert_eq!(&summary_6.values.add, &3);
            assert_eq!(&summary_6.children.as_ref().unwrap()[0].values.add, &3);
            assert_eq!(
                &summary_6.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[0]
                    .values
                    .add,
                &1
            );
            assert_eq!(
                &summary_6.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[1]
                    .values
                    .add,
                &1
            );

            // verify mul
            assert_eq!(&summary_6.values.mul, &3);
            assert_eq!(&summary_6.children.as_ref().unwrap()[0].values.mul, &3);
            assert_eq!(
                &summary_6.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[0]
                    .values
                    .mul,
                &0
            );
            assert_eq!(
                &summary_6.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[1]
                    .values
                    .mul,
                &2
            );

            // verify inv
            assert_eq!(&summary_6.values.inv, &1);
            assert_eq!(&summary_6.children.as_ref().unwrap()[0].values.inv, &1);
            assert_eq!(
                &summary_6.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[0]
                    .values
                    .inv,
                &1
            );
            assert_eq!(
                &summary_6.children.as_ref().unwrap()[0]
                    .children
                    .as_ref()
                    .unwrap()[1]
                    .values
                    .inv,
                &0
            );

            update_inv();
        }
        Tracker::end();

        let summary_7 = Tracker::summary();
        // verify add
        assert_eq!(&summary_7.values.add, &3);
        assert_eq!(&summary_7.children.as_ref().unwrap()[0].values.add, &3);
        assert_eq!(
            &summary_7.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[0]
                .values
                .add,
            &1
        );
        assert_eq!(
            &summary_7.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[1]
                .values
                .add,
            &1
        );

        // verify mul
        assert_eq!(&summary_7.values.mul, &3);
        assert_eq!(&summary_7.children.as_ref().unwrap()[0].values.mul, &3);
        assert_eq!(
            &summary_7.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[0]
                .values
                .mul,
            &0
        );
        assert_eq!(
            &summary_7.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[1]
                .values
                .mul,
            &2
        );

        // verify inv
        assert_eq!(&summary_7.values.inv, &2);
        assert_eq!(&summary_7.children.as_ref().unwrap()[0].values.inv, &2);
        assert_eq!(
            &summary_7.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[0]
                .values
                .inv,
            &1
        );
        assert_eq!(
            &summary_7.children.as_ref().unwrap()[0]
                .children
                .as_ref()
                .unwrap()[1]
                .values
                .inv,
            &0
        );

        Tracker::reset();
    }

    #[test]
    fn test_display() {
        Tracker::reset();
        gkr_sumcheck_squence();

        println!("{}", Tracker::summary());
        Tracker::reset();
    }
}
