use eopl::grammar::ProgramParser;
use eopl::interp::value_of_program;

#[derive(Debug)]
enum Expected {
    Num(i32),
    Error,
}

fn run_test(source: &str, expected: Expected) {
    let parser = ProgramParser::new();
    let program = parser.parse(source);

    match expected {
        Expected::Error => {
            let is_err = if let Ok(p) = program {
                value_of_program(&p).is_err()
            } else {
                true
            };
            assert!(is_err, "Source: {}", source);
        }
        Expected::Num(n) => {
            let p = program.unwrap_or_else(|_| panic!("Parse error: {}", source));
            let res = value_of_program(&p).unwrap_or_else(|_| panic!("Runtime error: {}", source));
            let num = res.as_num().unwrap();
            assert_eq!(num, n, "Source: {}", source);
        }
    }
}

macro_rules! eopl_test {
    ($name:ident, $code:expr, error) => {
        #[test]
        fn $name() {
            run_test($code, Expected::Error);
        }
    };
    ($name:ident, $code:expr, $num:expr) => {
        #[test]
        fn $name() {
            run_test($code, Expected::Num($num));
        }
    };
}

eopl_test!(positive_const, "11", 11);
eopl_test!(negative_const, "-33", -33);
eopl_test!(simple_arith_1, "-(44,33)", 11);

eopl_test!(nested_arith_left, "-(-(44,33),22)", -11);
eopl_test!(nested_arith_right, "-(55, -(22,11))", 44);

eopl_test!(test_var_1, "x", 10);
eopl_test!(test_var_2, "-(x,1)", 9);
eopl_test!(test_var_3, "-(1,x)", -9);

eopl_test!(test_unbound_var_1, "foo", error);
eopl_test!(test_unbound_var_2, "-(x,foo)", error);

eopl_test!(if_true, "if zero?(0) then 3 else 4", 3);
eopl_test!(if_false, "if zero?(1) then 3 else 4", 4);

eopl_test!(no_bool_to_diff_1, "-(zero?(0),1)", error);
eopl_test!(no_bool_to_diff_2, "-(1,zero?(0))", error);
eopl_test!(no_int_to_if, "if 1 then 2 else 3", error);

eopl_test!(if_eval_test_true, "if zero?(-(11,11)) then 3 else 4", 3);
eopl_test!(if_eval_test_false, "if zero?(-(11, 12)) then 3 else 4", 4);

eopl_test!(if_eval_test_true_2, "if zero?(-(11, 11)) then 3 else foo", 3);
eopl_test!(if_eval_test_false_2, "if zero?(-(11,12)) then foo else 4", 4);

eopl_test!(simple_let_1, "let x = 3 in x", 3);

eopl_test!(eval_let_body, "let x = 3 in -(x,1)", 2);
eopl_test!(eval_let_rhs, "let x = -(4,1) in -(x,1)", 2);

eopl_test!(simple_nested_let, "let x = 3 in let y = 4 in -(x,y)", -1);
eopl_test!(check_shadowing_in_body, "let x = 3 in let x = 4 in x", 4);
eopl_test!(check_shadowing_in_rhs, "let x = 3 in let x = -(x,1) in x", 2);

eopl_test!(apply_proc_in_rator_pos, "(proc(x) -(x,1) 30)", 29);
eopl_test!(apply_simple_proc, "let f = proc (x) -(x,1) in (f 30)", 29);
eopl_test!(let_to_proc_1, "(proc(f)(f 30) proc(x)-(x,1))", 29);

eopl_test!(nested_procs, "((proc (x) proc (y) -(x,y) 5) 6)", -1);
eopl_test!(nested_procs2, "let f = proc(x) proc (y) -(x,y) in ((f -(10,5)) 6)", -1);

eopl_test!(y_combinator_1, "
let fix = proc (f)
            let d = proc (x) proc (z) ((f (x x)) z)
            in proc (n) ((f (d d)) n)
in let t4m = proc (f) proc(x) if zero?(x) then 0 else -((f -(x,1)), -4)
in let times4 = (fix t4m)
   in (times4 3)", 12);

eopl_test!(simple_letrec_1, "letrec f(x) = -(x,1) in (f 33)", 32);
eopl_test!(simple_letrec_2, "letrec f(x) = if zero?(x) then 0 else -((f -(x,1)), -2) in (f 4)", 8);
eopl_test!(simple_letrec_3, "let m = -5 in letrec f(x) = if zero?(x) then 0 else -((f -(x,1)), m) in (f 4)", 20);

eopl_test!(fact_of_6, "letrec fact(x) = if zero?(x) then 1 else *(x, (fact -(x,1))) in (fact 6)", 720);

eopl_test!(ho_nested_letrecs, "
letrec even(odd) = proc(x) if zero?(x) then 1 else (odd -(x,1))
   in letrec odd(x) = if zero?(x) then 0 else ((even odd) -(x,1))
   in (odd 13)", 1);