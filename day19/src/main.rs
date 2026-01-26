use common::load;
use regex::Regex;
use std::collections::HashMap;

//a<2006:qkq,m>2090:A,rfg
#[derive(Debug)]
struct Rule {
    attribute: Option<char>,
    cmp: Option<char>,
    threshold: Option<i32>,
    workflow: String,
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn main() {
    println!("Day 19, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

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

    println!("Sum: {}", sum);
}

fn process_workflow(workflows: &HashMap<String, Vec<Rule>>, workflow: &str, part: &Part) -> String {
    if let Some(rules) = workflows.get(workflow) {
        for rule in rules {
            if let Some(attribute) = rule.attribute {
                let value = match attribute {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("Invalid attribute: {}", attribute),
                };
                let threshold = rule.threshold.unwrap();
                let cmp = rule.cmp.unwrap();
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
            let attribute = match1.as_str().chars().next();
            let cmp = captures.get(2).unwrap().as_str().chars().next();
            let threshold = captures.get(3).map(|m| m.as_str().parse().unwrap());
            let workflow = captures.get(4).unwrap().as_str().to_string();
            return Rule {
                attribute,
                cmp,
                threshold,
                workflow,
            };
        } else {
            let workflow = captures.get(5).unwrap().as_str().to_string();
            return Rule {
                attribute: None,
                cmp: None,
                threshold: None,
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
