#![feature(str_split_whitespace_remainder)]
#![feature(hash_raw_entry)]
#![feature(extend_one)]

use crate::days::day10::HoofIt;
use crate::days::day11::PlutonianPebbles;
use crate::days::day12::GardenGroups;
use crate::days::day13::ClawContraption;
use crate::days::day14::RestroomRedoubt;
use crate::days::day15::WarehouseWoes;
use crate::days::day16::ReindeerMaze;
use crate::days::day17::ChronospatialComputer;
use crate::days::day18::RAMRun;
use crate::days::day19::LinenLayout;
use crate::days::day1::HistorianHysteria;
use crate::days::day20::RaceCondition;
use crate::days::day21::KeypadConundrum;
use crate::days::day22::MonkeyMarket;
use crate::days::day23::LanParty;
use crate::days::day2::RedNosedReports;
use crate::days::day3::MullItOver;
use crate::days::day4::CeresSearch;
use crate::days::day5::PrintQueue;
use crate::days::day6::GuardGallivant;
use crate::days::day7::BridgeRepair;
use crate::days::day8::ResonantCollinearity;
use crate::days::day9::DiskFragmenter;
use crate::utils::aoc::AdventOfCode;

#[allow(unused_imports)]
use crate::utils::solution::SolveTest;

mod utils;
mod days;

fn main() {
    let mut aoc = AdventOfCode::default();
    aoc.add_solution(1, Box::new(HistorianHysteria::default()));
    aoc.add_solution(2, Box::new(RedNosedReports::default()));
    aoc.add_solution(3, Box::new(MullItOver::default()));
    aoc.add_solution(4, Box::new(CeresSearch::default()));
    aoc.add_solution(5, Box::new(PrintQueue::default()));
    aoc.add_solution(6, Box::new(GuardGallivant::default()));
    aoc.add_solution(7, Box::new(BridgeRepair::default()));
    aoc.add_solution(8, Box::new(ResonantCollinearity::default()));
    aoc.add_solution(9, Box::new(DiskFragmenter::default()));
    aoc.add_solution(10, Box::new(HoofIt::default()));
    aoc.add_solution(11, Box::new(PlutonianPebbles::default()));
    aoc.add_solution(12, Box::new(GardenGroups::default()));
    aoc.add_solution(13, Box::new(ClawContraption::default()));
    aoc.add_solution(14, Box::new(RestroomRedoubt::default()));
    aoc.add_solution(15, Box::new(WarehouseWoes::default()));
    aoc.add_solution(16, Box::new(ReindeerMaze::default()));
    aoc.add_solution(17, Box::new(ChronospatialComputer::default()));
    aoc.add_solution(18, Box::new(RAMRun::default()));
    aoc.add_solution(19, Box::new(LinenLayout::default()));
    aoc.add_solution(20, Box::new(RaceCondition::default()));
    aoc.add_solution(21, Box::new(KeypadConundrum::default()));
    aoc.add_solution(22, Box::new(MonkeyMarket::default()));
    aoc.add_solution(23, Box::new(LanParty::default()));

    //aoc.solve_day(23, SolveTest::All);
    aoc.solve_all();
}
