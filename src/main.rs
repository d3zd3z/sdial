//! Master "Speed Dial" lock simulation.

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};
use std::fmt;
use std::collections::BTreeMap;

fn main() {

    let matches = App::new("sdial")
        .version(crate_version!())
        .setting(AppSettings::GlobalVersion)
        .arg(Arg::with_name("max")
             .short("m")
             .long("max")
             .takes_value(true)
             .help(
                 "Maximum number of moves to consider (default 10)"))
        .arg(Arg::with_name("dups")
             .short("d")
             .long("dups")
             .help(
                 "Show all moves when they are duplicates"))
        .arg(Arg::with_name("all")
             .short("a")
             .long("all")
             .help(
                 "Show all combinations instead of just the best"))
        .arg(Arg::with_name("bests")
             .short("b")
             .long("bests")
             .help(
                 "Show all of the best candidates, not just the first"))
        .get_matches();

    let mut all = BTreeMap::new();

    // let max = 11;
    let max = matches.value_of("max").unwrap_or("10")
        .parse::<u64>().unwrap();

    let show_dups = matches.is_present("dups");
    let show_all = matches.is_present("all");
    let show_bests = matches.is_present("bests");

    for moves in 0..max {
        for binary in 0u64 .. (1 << 2*(moves+1)) {
            let mut lock = Lock::new();
            let mut tmp = binary;
            let mut seq = vec![];
            for _ in 0..(moves + 1) {
                lock.slide((tmp & 3) as u8);
                seq.push((tmp & 3) as u8);
                tmp >>= 2;
            }
            // if !all.contains_key(&lock) {
            //     // println!("Lock: {0} (move {2:01$b})", lock, 2*(moves+1), binary);
            //     println!("Lock: {} ({})", lock, MoveSeq(seq));
            // } else {
            //     // println!("         redundant: ({:?})", seq);
            // }
            let ent = all.entry(lock).or_insert_with(|| Target {
                count: 0,
                seq: MoveSeq(seq.clone()),
                all: vec![],
            });
            ent.all.push(MoveSeq(seq));
            ent.count += 1;
        }
    }

    println!("For up to {} moves", max);

    // Print out statistics about what we learned.
    println!("{} Uniques", all.len());

    // Count up all of the duplicates.
    let dups: usize = all.values().map(|x| x.count - 1).sum();
    println!("{} dups", dups);

    let mut moves: Vec<_> = all.iter().collect();
    moves.sort_by(|a, b| a.1.seq.0.len().cmp(&b.1.seq.0.len()));
    moves.sort_by_key(|m| m.1.count);

    if show_all {
        for &(lock, target) in &moves {
            println!("{} ({:4} target) {:-2} ({})", lock, target.count, target.seq.0.len(), target.seq);
            if show_dups && target.count > 1 {
                for mv in &target.all {
                    println!("   {}", mv);
                }
            }
        }
    }

    // Find the best move.
    let best_count = moves[0].1.count;
    for &(lock, target) in &moves {
        if target.count != best_count {
            break;
        }

        println!("Best: {} ({} target) ({})", lock, target.count, target.seq);
        if show_dups && target.count > 1 {
            for mv in &target.all {
                println!("   {}", mv);
            }
        }

        if !show_bests {
            break;
        }
    }
}

/// How we got to a state.
struct Target {
    /// How many moves (up to the max) arrive at this move.
    count: usize,
    /// The first sequence we encountered that got here.
    seq: MoveSeq,
    /// All of the sequences for this move.
    all: Vec<MoveSeq>,
}

/// The state of a single wheel within the lock.  The wheel can be in one
/// of 15 different positions.  To make the rest of this code easier, this
/// will be represented as a an integer 0-4 and a shift of -1, 0, or 1.
/// This helps because any given move will always leave a given wheel with
/// a particular shift, advancing the granular position if necessary.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
struct Wheel(u8);

impl Wheel {
    fn new(pos: u8, shift: i8) -> Wheel {
        Wheel(pos * 3 + (shift + 1) as u8)
    }

    fn set(&mut self, pos: u8, shift: i8) {
        self.0 = pos * 3 + (shift + 1) as u8;
    }

    fn pos(self) -> u8 {
        self.0 / 3
    }

    fn shift(self) -> i8 {
        (self.0 % 3) as i8 - 1
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.set(0, 0)
    }

    /// Advance this wheel to the given shift.
    fn advance(&mut self, shift: i8) {
        // If the current shift is strictly less than the desired shift,
        // only advance the shift.  Otherwise, the pos will have to be
        // advanced as well.
        if self.shift() < shift {
            let pos = self.pos();
            self.set(pos, shift);
        } else {
            let pos = if self.pos() == 4 {
                0
            } else {
                self.pos() + 1
            };
            self.set(pos, shift)
        }
    }
}

/// Display the wheel concisely.
impl fmt::Display for Wheel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pos = self.pos();
        let shift = self.shift();
        let sh = if shift < 0 {
            "<"
        } else if shift > 0 {
            ">"
        } else {
            "|"
        };
        write!(f, "{}{}", pos, sh)
    }
}

/// The entire lock contains 4 wheels.
#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
struct Lock {
    wheels: [Wheel; 4],
}

impl Lock {
    /// Construct a lock in the reset position.
    fn new() -> Lock {
        Lock {
            wheels: [Wheel::new(0, 0); 4],
        }
    }

    /// Perform a slide.  The directions are numbered according to their
    /// wheels (0-4).
    fn slide(&mut self, dir: u8) {
        self.wheels[prior(dir) as usize].advance(-1);
        self.wheels[dir as usize].advance(0);
        self.wheels[next(dir) as usize].advance(1);
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        for wh in &mut self.wheels {
            wh.reset();
        }
    }
}

fn prior(wheel: u8) -> u8 {
    if wheel == 0 {
        3
    } else {
        wheel - 1
    }
}

fn next(wheel: u8) -> u8 {
    if wheel == 3 {
        0
    } else {
        wheel + 1
    }
}

impl fmt::Display for Lock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for wh in &self.wheels {
            let ch = if first { '(' } else { ',' };
            first = false;
            write!(f, "{}{}", ch, wh)?;
        }
        write!(f, ")")
    }
}

struct MoveSeq(Vec<u8>);

impl fmt::Display for MoveSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &m in &self.0 {
            write!(f, "{}", match m {
                0 => 'U',
                1 => 'R',
                2 => 'D',
                3 => 'L',
                _ => unreachable!(),
            })?;
        }
        Ok(())
    }
}
