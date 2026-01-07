use regex::Regex;
use common::load;
use std::collections::{ HashMap, VecDeque };

const NUMBER_OF_BUTTON_PRESSES: i64 = 1000;

#[derive(Debug)]
struct Module<'a> {
    module_type: &'a str,
    sources: HashMap<&'a str, bool>,
    destinations: Vec<&'a str>,
    state: bool,
}

impl<'a> Module<'a> {
    fn new(module_type: &'a str, destinations: Vec<&'a str>) -> Self {
        Self {
            module_type,
            sources: HashMap::new(),
            destinations,
            state: false,
        }
    }
}

fn main() {
    println!("Day 20, part {}", if cfg!(feature="part2") { "2" } else { "1" });

    let lines = load::lines().unwrap();
    let mut modules = load_modules(&lines);

//    println!("Modules: {:?}", modules);

    let mut low_count: i64 = 0;
    let mut high_count: i64 = 0;

    // Hit the button a bunch of times
    for _ in 0..NUMBER_OF_BUTTON_PRESSES {
        let mut queue: VecDeque<(&str, &str, bool)> = VecDeque::new();
        queue.push_back(("", "broadcaster", false));
        while !queue.is_empty() {
            let (from, to, input) = queue.pop_front().unwrap();
            if input == false {
                low_count += 1;
            } else {
                high_count += 1;
            }    
            let mut optional_output: Option<bool> = None;
            if let Some(module) = modules.get_mut(to) {
                if to == "broadcaster" {
                    optional_output = Some(input);
                } else {
                    match module.module_type {
                        "%" => {
                            if input == false {
                                module.state = !module.state;
                                optional_output = Some(module.state);
                            }
                        },
                        "&" => {
                            module.sources.insert(from, input);
                            optional_output = Some(!module.sources.iter().all(|(_, pulse)| *pulse));
                        },
                        _ => {
                            panic!("Unknown module type: {}", module.module_type);
                        }
                    }
                }

                // Queue the outputted pulses
                if let Some(output) = optional_output {
                    for destination in &module.destinations {
                        queue.push_back((to, destination, output));
                    }
                }
            } else {
                println!("Module not found: {}", to);
            }
        }
    }

    println!("Low count: {}", low_count);
    println!("High count: {}", high_count);
    println!("Answer: {}", low_count * high_count);
}

fn load_modules<'a>(lines: &'a Vec<String>) -> HashMap<&'a str, Module<'a>>{
    let mut modules: HashMap<&'a str, Module> = HashMap::new();

    // Create the modules from the input
    for line in lines {
        let re = Regex::new(r"^([%&]?)(\w+)\s*->\s*([,\w\s]+)$").unwrap();
        if let Some(captures) = re.captures(line) {
            let module_type = captures.get(1).map_or("", |m| m.as_str());
            let name = captures.get(2).unwrap().as_str();
            let destinations = captures.get(3).unwrap().as_str().split(",").map(|s| s.trim()).collect();
            modules.insert(name, Module::new(module_type, destinations));
        } else {
            println!("Can't parse line: {}", &line);
        }
    }

    // Get the sources for each module
    let mut sources_by_destination: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    for (name, module) in &modules {
        for destination in &module.destinations {
            if modules.get(destination).is_some() {
                sources_by_destination.entry(destination).or_insert_with(HashMap::new).insert(name, false);
            } else {
                println!("Destination module not found: {}", destination);
            }
        }
    }

    // Save each module's sources
    for (name, module) in &mut modules {
        module.sources = if let Some(sources) = sources_by_destination.get(name) { sources.clone() } else { HashMap::new() };
    }

    modules
}
