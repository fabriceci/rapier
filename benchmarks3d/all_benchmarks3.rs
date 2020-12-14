#![allow(dead_code)]

extern crate nalgebra as na;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use inflector::Inflector;

use rapier_testbed3d::Testbed;
use std::cmp::Ordering;

mod balls3;
mod boxes3;
mod capsules3;
mod compound3;
mod heightfield3;
mod joint_ball3;
mod joint_fixed3;
mod joint_prismatic3;
mod joint_revolute3;
mod keva3;
mod pyramid3;
mod stacks3;
mod trimesh3;

enum Command {
    Run(String),
    List,
    RunAll,
}

fn parse_command_line() -> Command {
    let mut args = std::env::args();

    while let Some(arg) = args.next() {
        if &arg[..] == "--example" {
            return Command::Run(args.next().unwrap_or(String::new()));
        } else if &arg[..] == "--list" {
            return Command::List;
        }
    }

    Command::RunAll
}

pub fn main() {
    let command = parse_command_line();

    let mut builders: Vec<(_, fn(&mut Testbed))> = vec![
        ("Balls", balls3::init_world),
        ("Boxes", boxes3::init_world),
        ("Capsules", capsules3::init_world),
        ("Compound", compound3::init_world),
        ("Heightfield", heightfield3::init_world),
        ("Stacks", stacks3::init_world),
        ("Pyramid", pyramid3::init_world),
        ("TriMesh", trimesh3::init_world),
        ("Joint ball", joint_ball3::init_world),
        ("Joint fixed", joint_fixed3::init_world),
        ("Joint revolute", joint_revolute3::init_world),
        ("Joint prismatic", joint_prismatic3::init_world),
        ("Keva tower", keva3::init_world),
    ];

    // Lexicographic sort, with stress tests moved at the end of the list.
    builders.sort_by(|a, b| match (a.0.starts_with("("), b.0.starts_with("(")) {
        (true, true) | (false, false) => a.0.cmp(b.0),
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
    });

    match command {
        Command::Run(demo) => {
            if let Some(i) = builders
                .iter()
                .position(|builder| builder.0.to_camel_case().as_str() == demo.as_str())
            {
                Testbed::from_builders(0, vec![builders[i]]).run()
            } else {
                eprintln!("Invalid example to run provided: '{}'", demo);
            }
        }
        Command::RunAll => Testbed::from_builders(0, builders).run(),
        Command::List => {
            for builder in &builders {
                println!("{}", builder.0.to_camel_case())
            }
        }
    }
}
