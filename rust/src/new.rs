use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() {
  let args: Vec<String> = env::args().collect();
  let solution_no_str = &args[1];
  let solution_no: usize = solution_no_str.parse().unwrap();

  create_new_solution(solution_no);
  update_mod(solution_no);
}

fn create_new_solution(no: usize) {
  let file_path = format!("src/solutions/s{:>04}.rs", no);
  let mut file = File::create(file_path).unwrap();
  let tmpl = format!(
    "use proconio::input;

  // Solution for Project Euler problem {}
  // Copyright michimani All rights reserved.
  //
  // https://projecteuler.net/problem={}
  pub fn solve() {{
    input!{{
      n: usize,
    }}

    let mut answer = 0;

    println!(\"answer is {{}}\", answer);
  }}
  ",
    no, no
  );

  file.write_all(tmpl.as_bytes()).unwrap();
}

fn update_mod(no: usize) {
  let new_line = format!("pub mod s{:>04};\n", no);
  let mod_path = "src/solutions/mod.rs";
  let mut mod_file = OpenOptions::new().append(true).open(mod_path).unwrap();
  mod_file.write_all(new_line.as_bytes()).unwrap();
}
