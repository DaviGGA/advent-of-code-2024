mod ex_1;

fn main() {
    let lists = ex_1::get_ex1_input().unwrap();
    println!("{}", ex_1::historian_hysteria(&lists.0, &lists.1));
}
