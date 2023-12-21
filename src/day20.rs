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

type RefModule = Rc<RefCell<Module>>;
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
        println!("{} {:?}", self.name, self.kind);
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
            let input_index = self.inputs.iter().position(|x| x.as_ptr() == source.as_ptr()).unwrap();
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

    fn pulse(module: RefModule, value: bool, source: RefModule, queue: &mut VecDeque<AddressedPulse>) {
        let mut mm = module.as_ref().borrow_mut();
        if let Some(new_v) = mm.perform(value, source) {
            for output in mm.outputs.iter() {
                //println!("Pushing {} -> {} ({:?})", new_v, output.as_ref().borrow().name, output.as_ref().borrow().kind);
                queue.push_back((module.clone(), output.clone(), new_v));
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

fn day20_inner(input_fname: &str) -> i64 {
    let data = std::fs::read_to_string(input_fname).unwrap();
    let mut modules = vec![];
    let mut modoutputs = vec![];
    let mut namemap = HashMap::new();

    // broadcaster -> a, b, c
// %a -> b
// %b -> c
// %c -> inv
// &inv -> a

    let outmod = Rc::new(RefCell::new(Module::new("OUTPUT", ModuleKind::Debug)));
    modules.push(outmod.clone());
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

        let module = Rc::new(RefCell::new(Module::new(name, kind)));
        let module_index = if let Some(index) = namemap.get(name) {
            *index
        } else {
            let index = modules.len();
            namemap.insert(name.to_string(), index);
            modules.push(Rc::new(RefCell::new(Module::new("PLACEHOLDER", ModuleKind::Debug))));
            modoutputs.push(vec![]);
            index
        };

        modules[module_index] = module;
        modoutputs[module_index] = output_names;
    }

    // fill in the outputs with actual links
    for i in 0..modules.len() {
        for &outname in modoutputs[i].iter() {
            let mut mm = modules[i].as_ref().borrow_mut();
            if let Some(outindex) = namemap.get(outname) {
                mm.outputs.push(modules[*outindex].clone());
                modules[*outindex].as_ref().borrow_mut().push_input(modules[i].clone());
            } else {
                println!("Using dummy output for {}", outname);
                mm.outputs.push(outmod.clone());
            }
        }
    }

    let broadcaster = modules[namemap["broadcaster"]].clone();

    let mut low_count: i64 = 0;
    let mut high_count: i64 = 0;
    let mut pulse_queue = VecDeque::new();

    for i in 0..1000 {
        pulse_queue.push_front((broadcaster.clone(), broadcaster.clone(), false));
        debug_println!("==== button press =====");

        while let Some(pulse) = pulse_queue.pop_front() {
            debug_println!("Pulse: {} -{}-> {}     ({:?} -> {:?})", pulse.0.as_ref().borrow().name, if pulse.2 { "high" } else { "low" },
                pulse.1.as_ref().borrow().name, pulse.0.as_ref().borrow().kind, pulse.1.as_ref().borrow().kind);
            if pulse.2 { high_count += 1; } else { low_count += 1; }
            Module::pulse(pulse.1, pulse.2, pulse.0, &mut pulse_queue);
        }
    }

    println!("Low: {}, High: {}", low_count, high_count);
    low_count * high_count
}

fn main() {
    let r = day20_inner("inputs/day20-sample-2.txt");
    println!("Result: {}", r);

    println!("===== Real =====");
    let r = day20_inner("inputs/day20.txt");
    println!("Result: {}", r);
}