#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;

extern crate num;

extern crate petgraph;

extern crate rayon;

extern crate rand;

pub mod problems {
    pub mod problem_001;
    pub mod problem_002;
    pub mod problem_003;
    pub mod problem_004;
    pub mod problem_005;
    pub mod problem_006;
    pub mod problem_007;
    pub mod problem_008;
    pub mod problem_009;
    pub mod problem_010;
    pub mod problem_011;
    pub mod problem_012;
    pub mod problem_013;
    pub mod problem_014;
    pub mod problem_015;
    pub mod problem_016;
    pub mod problem_018;
    pub mod problem_020;
    pub mod problem_024;
    pub mod problem_025;
    pub mod problem_026;
    pub mod problem_030;
    pub mod problem_031;
    pub mod problem_032;
    pub mod problem_034;
    pub mod problem_039;
    pub mod problem_040;
    pub mod problem_044;
    pub mod problem_045;
    pub mod problem_047;
    pub mod problem_048;
    pub mod problem_050;
    pub mod problem_052;
    pub mod problem_056;
    pub mod problem_058;
    pub mod problem_059;
    pub mod problem_075;
    pub mod problem_078;
    pub mod problem_081;
    pub mod problem_082;
    pub mod problem_083;
    pub mod problem_086;
    pub mod problem_087;
    pub mod problem_089;
    pub mod problem_093;
    pub mod problem_097;
    pub mod problem_099;
    pub mod problem_104;
    pub mod problem_114;
    pub mod problem_119;
    pub mod problem_121;
    pub mod problem_122;
    pub mod problem_125;
    pub mod problem_145;
    pub mod problem_197;
    pub mod problem_205;
}

pub mod util {
    pub mod coins;
    pub mod grid;
    pub mod iter;
    pub mod math;
    pub mod prime;
    pub mod triangle;
}
