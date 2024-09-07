#![allow(unused_imports, unused_variables, dead_code, unused_parens, non_snake_case)]
use core::num;
use std::{cmp::{min,max}, collections::HashMap, collections::HashSet, str::FromStr, ops::BitAnd, borrow::BorrowMut, u8};
use std::collections::VecDeque;
use itertools::Itertools;
use regex::Regex;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use typed_arena::Arena;
use bumpalo::Bump;

use debug_print::{debug_print, debug_println, debug_eprint, debug_eprintln};

mod helpers;
use helpers::*;

#[derive(Debug, PartialEq)]
enum ModuleKind {
    Debug,
    Broadcast,
    FlipFlop,
    Conjunction,
}

type RefModule = *mut Module;
type AddressedPulse = (RefModule, RefModule, bool); // src, dst, value

struct Module {
    name: String,
    kind: ModuleKind,
    flip_storage: bool,
    conj_storage: Vec<bool>,

    inputs: Vec<RefModule>,
    outputs: Vec<RefModule>,
}

impl Module {
    fn new(name: &str, kind: ModuleKind) -> Self {
        Self {
            name: name.to_string(),
            kind,
            flip_storage: false,
            conj_storage: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn perform(&mut self, value: bool, source: RefModule) -> Option<bool> {
        if self.kind == ModuleKind::Debug {
            debug_println!("Debug[{}]: {}", self.name, value);
            return Some(value);
        }
        
        if self.kind == ModuleKind::Broadcast {
            return Some(value);
        }
        
        if self.kind == ModuleKind::FlipFlop {
            if value { return None; }
            self.flip_storage = !self.flip_storage;
            return Some(self.flip_storage);
        }

        if self.kind == ModuleKind::Conjunction {
            let input_index = self.inputs.iter().position(|x| *x == source).unwrap();
            self.conj_storage[input_index] = value;
            for val in &self.conj_storage {
                if !val {
                    return Some(true);
                }
            }

            // all storage is true
            return Some(false);
        }

        panic!()
    }

    fn pulse(module: RefModule, value: bool, source: RefModule, queue: &mut Vec<AddressedPulse>) {
        unsafe {
            let mm = module.as_mut().unwrap();
            if let Some(new_v) = mm.perform(value, source) {
                for output in mm.outputs.iter() {
                    //println!("Pushing {} -> {} ({:?})", new_v, output.as_ref().borrow().name, output.as_ref().borrow().kind);
                    queue.push((module.clone(), output.clone(), new_v));
                }
            }
        }
    }

    fn push_input(&mut self, input: RefModule) {
        self.inputs.push(input);
        if self.kind == ModuleKind::Conjunction {
            self.conj_storage.push(false);
        }
    }
}

fn modules_state(modules: &Vec<RefModule>) -> String {
    let mut result = String::new();
    for module in modules.iter() {
        unsafe {
            let mm = module.as_ref().unwrap();
            if mm.kind == ModuleKind::Debug {
                continue;
            }
            //result.push_str(&format!("{}:", mm.name));
            if mm.kind == ModuleKind::Broadcast {
                result.push_str(">");
            } else if mm.kind == ModuleKind::FlipFlop {
                result.push_str(if mm.flip_storage { "+" } else { "_" });
            } else if mm.kind == ModuleKind::Conjunction {
                result.push_str(&mm.conj_storage.iter().map(|x| if *x { '*' } else { '.' }).collect::<String>());
            }
            //result.push_str(" ");
        }
    }
    result
}

fn day20_inner(input_fname: &str) -> i64 {
    let data = std::fs::read_to_string(input_fname).unwrap();
    let mut modules = vec![];
    let mut modoutputs = vec![];
    let mut namemap = HashMap::new();

    let bumper = Bump::new();

    // broadcaster -> a, b, c
// %a -> b
// %b -> c
// %c -> inv
// &inv -> a

    let outmod = bumper.alloc(Module::new("OUTPUT", ModuleKind::Debug)) as RefModule;
    modules.push(outmod);
    namemap.insert("rx".to_string(), 0);
    modoutputs.push(vec![]);

    for line in data.lines() {
        let mut parts = line.split(" -> ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let output_names = right.split(", ").collect::<Vec<_>>();
        let name = if left == "broadcaster" { left } else { &left[1..] };

        let kind = match &left[0..1] {
            "b" => ModuleKind::Broadcast,
            "%" => ModuleKind::FlipFlop,
            "&" => ModuleKind::Conjunction,
            _ => panic!(),
        };

        let module = bumper.alloc(Module::new(name, kind)) as RefModule;
        let module_index = if let Some(index) = namemap.get(name) {
            *index
        } else {
            let index = modules.len();
            namemap.insert(name.to_string(), index);
            modules.push(bumper.alloc(Module::new("PLACEHOLDER", ModuleKind::Debug)) as RefModule);
            modoutputs.push(vec![]);
            index
        };

        modules[module_index] = module;
        modoutputs[module_index] = output_names;
    }

    // fill in the outputs with actual links
    for i in 0..modules.len() {
        for &outname in modoutputs[i].iter() {
            unsafe {
                let mm = modules[i].as_mut().unwrap();
                if let Some(outindex) = namemap.get(outname) {
                    mm.outputs.push(modules[*outindex]);
                    modules[*outindex].as_mut().unwrap().push_input(modules[i].clone());
                } else {
                    println!("Using dummy output for {}", outname);
                    mm.outputs.push(outmod.clone());
                }
            }
        }
    }

    let broadcaster = modules[namemap["broadcaster"]];

    /*
    let mut rx_input_queue = VecDeque::new();
    let mut seen_q = HashSet::new();
    rx_input_queue.push_back(outmod);
    while let Some(module) = rx_input_queue.pop_front() {
        if seen_q.contains(&module) {
            continue;
        }
        seen_q.insert(module.clone());
        let mm = unsafe { module.as_ref().unwrap() };
        println!("rx_input_queue: {} <- {:?}", mm.name, mm.inputs.iter().map(|x| &unsafe { x.as_ref().unwrap() }.name).collect::<Vec<_>>());
        for input in mm.inputs.iter() {
            rx_input_queue.push_back(input.clone());
        }
    }

    panic!("done");
    */

    let mut totals = vec![];

    for &node_start in unsafe { broadcaster.as_ref().unwrap() }.outputs.iter() {
        let mut node = node_start;
        let mut num: i64 = 0;

        for i in 0.. {
            let node_outputs = unsafe { node.as_ref().unwrap() }.outputs.clone();
            let flip_flops = node_outputs.iter()
                .filter(|x| unsafe { x.as_ref().unwrap() }.kind == ModuleKind::FlipFlop)
                .collect_vec();
            match flip_flops.len() {
                0 => {
                    num += 1 << i;
                    break;
                },
                1 => {
                    if node_outputs.len() > 1 {
                        num += 1 << i;
                    }
                    node = flip_flops[0].clone();
                },
                _ => panic!(),
            }
        }

        totals.push(num);
    }

    println!("Totals: {:?}", totals);
    return totals.iter().product::<i64>();



    let mut low_count: i64 = 0;
    let mut high_count: i64 = 0;
    //let mut pulse_queue = VecDeque::with_capacity(1000);
    let mut pulse_queue = vec![];

    let mut i: i64 = 0;
    loop {
        let mut q_index = 0;
        pulse_queue.clear();
        pulse_queue.push((broadcaster, broadcaster, false));
        i += 1;
        println!("{}: {}", i, modules_state(&modules));

        if i % 1_000_000 == 0 {
            println!("... {} (deque {})", i, pulse_queue.len());
        }

        while q_index < pulse_queue.len() {
            let pulse = pulse_queue[q_index];
            if pulse.1 == outmod && pulse.2 == false {
                debug_println!("Output: {}", i);
                panic!("Output: {}", i);
            }

            unsafe {
                debug_println!("Pulse: {} -{}-> {}", pulse.0.as_ref().unwrap().name, if pulse.2 { "high" } else { "low" },
                    pulse.1.as_ref().unwrap().name);
            }
            if pulse.2 { high_count += 1; } else { low_count += 1; }
            Module::pulse(pulse.1, pulse.2, pulse.0, &mut pulse_queue);

            q_index += 1;
            //println!("pulse_queue.len() {} q_index {}\n", pulse_queue.len(), q_index);
        }
    }

    //println!("Low: {}, High: {}", low_count, high_count);
    //low_count * high_count
}

fn main() {
    //let r = day20_inner("inputs/day20-sample-2.txt");
    //println!("Result: {}", r);

    println!("===== Real =====");
    let r = day20_inner("inputs/day20.txt");
    println!("Result: {}", r);
}