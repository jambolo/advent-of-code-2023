use common::load;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Module {
    module_type: String,
    sources: HashMap<String, bool>,
    destinations: Vec<String>,
    state: bool,
}

impl Module {
    fn new(module_type: &str, destinations: &[String]) -> Self {
        Self {
            module_type: module_type.to_string(),
            sources: HashMap::new(),
            destinations: destinations.to_vec(),
            state: false,
        }
    }
}

fn main() {
    println!("Day 20, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines().unwrap();
    let mut modules = load_modules(&lines);

    #[cfg(not(feature = "part2"))]
    part1(&mut modules);

    #[cfg(feature = "part2")]
    part2(&mut modules);
}

fn load_modules(lines: &[String]) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();

    // First add the output and rx modules They are not defined in the input but modules output to them
    modules.insert("output".to_string(), Module::new("*", &[]));
    modules.insert("rx".to_string(), Module::new("*", &[]));

    // Create the modules from the input
    let re = Regex::new(r"^([%&]?)(\w+)\s*->\s*([,\w\s]+)$").expect("Invalid regex");
    for line in lines {
        let captures = re.captures(line).unwrap_or_else(|| panic!("Failed to parse line: {}", line));
        let module_type = captures.get(1).map_or("X", |m| m.as_str());
        let name = captures.get(2).expect("Failed to get module name").as_str().to_string();
        let destinations = captures
            .get(3)
            .expect("Failed to get destinations")
            .as_str()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        if name == "broadcaster" {
            modules.insert(name, Module::new("<", &destinations));
        } else {
            modules.insert(name, Module::new(module_type, &destinations));
        }
    }

    // Get the sources for each module
    let mut sources_by_destination: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for (name, module) in &modules {
        for destination in &module.destinations {
            if modules.get(destination).is_none() {
                panic!("Module {} has an unknown destination: {}", name, destination);
            }
            sources_by_destination
                .entry(destination.clone())
                .or_insert_with(HashMap::new)
                .insert(name.clone(), false);
        }
    }

    // Save each module's sources
    for (name, module) in &mut modules {
        module.sources = if let Some(sources) = sources_by_destination.get(name) {
            sources.clone()
        } else {
            HashMap::new()
        };
    }

    modules
}

#[cfg(not(feature = "part2"))]
fn part1(modules: &mut HashMap<String, Module>) {
    const NUMBER_OF_BUTTON_PRESSES: i64 = 1000;

    let mut low_count: i64 = 0;
    let mut high_count: i64 = 0;

    // Hit the button a bunch of times
    for _ in 0..NUMBER_OF_BUTTON_PRESSES {
        let (low, high) = run(modules);
        low_count += low;
        high_count += high;
    }

    println!("Answer: {}", low_count * high_count);
}

#[cfg(feature = "part2")]
fn part2(modules: &mut HashMap<String, Module>) {
    let mut count: i64 = 0;
    let mut vd_triggered: Option<i64> = None;
    let mut ns_triggered: Option<i64> = None;
    let mut bh_triggered: Option<i64> = None;
    let mut dl_triggered: Option<i64> = None;
    loop {
        run(modules);
        count += 1;
        if vd_triggered.is_none() && modules.get("vd").unwrap().state {
            modules.get_mut("vd").unwrap().state = false; // reset vd
            vd_triggered = Some(count);
        }
        if ns_triggered.is_none() && modules.get("ns").unwrap().state {
            modules.get_mut("ns").unwrap().state = false; // reset ns
            ns_triggered = Some(count);
        }
        if bh_triggered.is_none() && modules.get("bh").unwrap().state {
            modules.get_mut("bh").unwrap().state = false; // reset bh
            bh_triggered = Some(count);
        }
        if dl_triggered.is_none() && modules.get("dl").unwrap().state {
            modules.get_mut("dl").unwrap().state = false; // reset dl
            dl_triggered = Some(count);
        }
        // Get the product of all triggered counts or None if any didn't trigger
        let product = vd_triggered.and_then(|vd| {
            ns_triggered.and_then(|ns| bh_triggered.and_then(|bh| dl_triggered.and_then(|dl| Some(vd * ns * bh * dl))))
        });
        if let Some(p) = product {
            println!("Product of triggered counts: {}", p);
            break;
        }
    }
}

fn run(modules: &mut HashMap<String, Module>) -> (i64, i64) {
    let mut low_count: i64 = 0;
    let mut high_count: i64 = 0;

    let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();
    queue.push_back(("".to_string(), "broadcaster".to_string(), false));

    while let Some((from, name, input)) = queue.pop_front() {
        if input {
            high_count += 1;
        } else {
            low_count += 1;
        }
        let module = modules
            .get_mut(&name)
            .unwrap_or_else(|| panic!("Stepping an unknown module: {}", name));
        if let Some(output) = step(module, input, &from) {
            propagate(&mut queue, module, &name, output);
        }
    }

    (low_count, high_count)
}

fn step(module: &mut Module, input: bool, from: &str) -> Option<bool> {
    let mut output: Option<bool> = None;
    match module.module_type.as_str() {
        "%" => {
            // Flip-flop (outputs only if input is false)
            if input == false {
                module.state = !module.state;
                output = Some(module.state);
            }
        }
        "&" => {
            // NAND
            module.sources.insert(from.to_string(), input);
            let state = !module.sources.iter().all(|(_, pulse)| *pulse);
            if state && !module.state {
                module.state = true;
            }
            output = Some(state);
        }
        "<" => {
            // Broadcaster
            module.state = input;
            output = Some(input);
        }
        "*" => {
            // Detectors (rx and output only)
            module.state = !input; // Goes high when input is false
                                   // no output
        }
        _ => {
            panic!("Unknown module type: {}", module.module_type);
        }
    }

    output
}

fn propagate(queue: &mut VecDeque<(String, String, bool)>, module: &Module, from: &str, output: bool) {
    for to in &module.destinations {
        queue.push_back((from.to_string(), to.clone(), output));
    }
}
