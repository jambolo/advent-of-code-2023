use common::load;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

type OptionalRange = Option<(i32, i32)>;

#[derive(Debug)]
struct Constraint {
    attribute: char,   // x, m, a, s
    cmp: char,         // <, >
    threshold: i32,
}
#[derive(Debug)]
struct Rule {
    constraint: Option<Constraint>,
    workflow: String,           // A, R, or another workflow name
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
#[derive(Debug, Clone)]
struct XmasRange {
    x: (i32, i32),  // [lower, upper)
    m: (i32, i32),  // [lower, upper)
    a: (i32, i32),  // [lower, upper)
    s: (i32, i32),  // [lower, upper)
}

fn main() {
    println!("=== Day 19, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines().unwrap();

    // Parse each line as a workflow until an empty line is found.
    let mut workflows = HashMap::new();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].is_empty() {
            break;
        }
        let (name, rules) = parse_workflow(&lines[i]);
        workflows.insert(name, rules);
        i += 1;
    }

    // Parse the remaining lines as part descriptions
    let mut parts = Vec::new();
    i += 1;
    while i < lines.len() {
        parts.push(parse_part(&lines[i]));
        i += 1;
    }

    let result = if cfg!(feature = "part2") {
        part2(workflows)
    } else {
        part1(workflows, parts)
    };
    println!("Result: {}", result)
}

fn part1(workflows: HashMap<String, Vec<Rule>>, parts: Vec<Part>) -> i64 {
    let mut sum: i64 = 0;

    // Run each part through the workflow
    for part in parts {
        let mut workflow = process_workflow(&workflows, "in", &part);
        while workflow != "A" && workflow != "R" {
            workflow = process_workflow(&workflows, workflow.as_str(), &part);
        }

        if workflow == "A" {
            sum += (part.x + part.m + part.a + part.s) as i64;
        } else {
            debug_assert!(workflow == "R");
        }
    }
    sum
}

fn part2(workflows: HashMap<String, Vec<Rule>>) -> i64 {
    let mut count:i64 = 0;

    let mut queue: VecDeque<(String, XmasRange)> = VecDeque::new();
    queue.push_back(("in".to_string(), XmasRange {
        x: (1, 4001),   // [1, 4000]
        m: (1, 4001),   // [1, 4000]
        a: (1, 4001),   // [1, 4000]
        s: (1, 4001),   // [1, 4000]
    }));

    let mut process_count = 0;
    while !queue.is_empty() {
        let (workflow, initial_range) = queue.pop_front().unwrap();
        if workflow == "A" {
            count += range_combinations(&initial_range);
            continue;
        } else if workflow == "R" {
            process_count += 1;
            if process_count % 1000 == 0 {
                println!("Processed {} workflows, queue size: {}, count: {}", process_count, queue.len(), count);
            }
            continue;
        }
        let rules = workflows.get(&workflow).unwrap();
        let mut range = initial_range;
        for rule in rules {
            let (r0, r1) = split_xmas_range(&range, &rule.constraint);
            if let Some(sub_range) = r0 {
                queue.push_back((rule.workflow.clone(), sub_range));
            }
            if let Some(remainder) = r1 {
                range = remainder;
            } else {
                break;
            }
        }
        process_count += 1;
        if process_count % 1000 == 0 {
            println!("Processed {} workflows, queue size: {}, count: {}", process_count, queue.len(), count);
        }
    }
    count
}

fn range_combinations(range: &XmasRange) -> i64 {
    (range.x.1 - range.x.0) as i64 *
    (range.m.1 - range.m.0) as i64 *
    (range.a.1 - range.a.0) as i64 *
    (range.s.1 - range.s.0) as i64
}

fn split_xmas_range(range: &XmasRange, constraint: &Option<Constraint>) -> (Option<XmasRange>, Option<XmasRange>) {
    if let Some(c) = constraint {
        match c.attribute{
            'x' => {
                let (x0, x1) = split_range(range.x, c.cmp, c.threshold);
                let sub_range = x0.map(|x| XmasRange { x, m: range.m, a: range.a, s: range.s });
                let remainder = x1.map(|x| XmasRange { x, m: range.m, a: range.a, s: range.s });
                (sub_range, remainder)
            }
            'm' => {
                let (m0, m1) = split_range(range.m, c.cmp, c.threshold);
                let sub_range = m0.map(|m| XmasRange { x: range.x, m, a: range.a, s: range.s });
                let remainder = m1.map(|m| XmasRange { x: range.x, m, a: range.a, s: range.s });
                (sub_range, remainder)
            }
            'a' => {
                let (a0, a1) = split_range(range.a, c.cmp, c.threshold);
                let sub_range = a0.map(|a| XmasRange { x: range.x, m: range.m, a, s: range.s });
                let remainder = a1.map(|a| XmasRange { x: range.x, m: range.m, a, s: range.s });
                (sub_range, remainder)
            }
            's' => {
                let (s0, s1) = split_range(range.s, c.cmp, c.threshold);
                let sub_range = s0.map(|s| XmasRange { x: range.x, m: range.m, a: range.a, s });
                let remainder = s1.map(|s| XmasRange { x: range.x, m: range.m, a: range.a, s });
                (sub_range, remainder)
            }
            _ => panic!("Invalid attribute: {}", c.attribute),
        }
    } else {
        (Some(range.clone()), None)
    }
}

fn split_range(range: (i32, i32), cmp: char, threshold: i32) -> (OptionalRange, OptionalRange) {
    if cmp == '<' && threshold > range.0 {
        let upper = range.1.min(threshold);
        (Some((range.0, upper)), if upper < range.1 { Some((upper, range.1)) } else { None })
    } else if cmp == '>' && threshold + 1 < range.1 {
        let lower = range.0.max(threshold + 1);
        (Some((lower, range.1)), if lower > range.0 { Some((range.0, lower)) } else { None })
    } else {
        (None, Some(range)) // No split, return original range as remainder
    }
}
fn process_workflow(workflows: &HashMap<String, Vec<Rule>>, workflow: &str, part: &Part) -> String {
    if let Some(rules) = workflows.get(workflow) {
        for rule in rules {
            if let Some(constraint) = &rule.constraint {
                let value = match constraint.attribute {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("Invalid attribute: {}", constraint.attribute),
                };
                let threshold = constraint.threshold;
                let cmp = constraint.cmp;
                match cmp {
                    '<' => {
                        if value < threshold {
                            return rule.workflow.clone();
                        }
                    }
                    '>' => {
                        if value > threshold {
                            return rule.workflow.clone();
                        }
                    }
                    _ => panic!("Invalid comparison: {}", cmp),
                }
            } else {
                return rule.workflow.clone();
            }
        }
    } else {
        panic!("No workflow {:?}", workflow);
    }
    panic!("No rule matched for workflow {:?}", workflow);
}

fn parse_workflow(line: &str) -> (String, Vec<Rule>) {
    let workflow_re = Regex::new(r"^(\w+)\s*\{([^}]*)\}$").unwrap();
    if let Some(captures) = workflow_re.captures(line) {
        let name = captures.get(1).unwrap().as_str().to_string();
        let rule_strings: Vec<&str> = captures.get(2).unwrap().as_str().split(',').map(|s| s.trim()).collect();
        let rules = rule_strings.into_iter().map(parse_rule).collect();
        (name, rules)
    } else {
        panic!("Invalid workflow string: {}", line);
    }
}

fn parse_rule(s: &str) -> Rule {
    let rule_re = Regex::new(r"^([xmas])([<>])(\d+):(\w+)|(\w+)$").unwrap();
    if let Some(captures) = rule_re.captures(s) {
        if let Some(match1) = captures.get(1) {
            let attribute = match1.as_str().chars().next().unwrap();
            let cmp = captures.get(2).unwrap().as_str().chars().next().unwrap();
            let threshold = captures.get(3).map(|m| m.as_str().parse().unwrap()).unwrap();
            let workflow = captures.get(4).unwrap().as_str().to_string();
            return Rule {
                constraint: Some(Constraint {
                    attribute,
                    cmp,
                    threshold,
                }),
                workflow,
            };
        } else {
            let workflow = captures.get(5).unwrap().as_str().to_string();
            return Rule {
                constraint: None,
                workflow,
            };
        }
    }
    panic!("Invalid rule string: {}", s);
}

fn parse_part(line: &str) -> Part {
    let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    if let Some(captures) = re.captures(line) {
        let x = captures.get(1).unwrap().as_str().parse().unwrap();
        let m = captures.get(2).unwrap().as_str().parse().unwrap();
        let a = captures.get(3).unwrap().as_str().parse().unwrap();
        let s = captures.get(4).unwrap().as_str().parse().unwrap();
        Part { x, m, a, s }
    } else {
        panic!("Invalid part string: {}", line);
    }
}
