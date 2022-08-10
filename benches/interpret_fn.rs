use bencher::{benchmark_group, benchmark_main, Bencher};
use runthrough::interpret::{interpret_fn, postfix_eval};

fn a(bench: &mut Bencher) {
    let input = "2 * x pow 3";
    bench.iter(|| {
        let postfix = interpret_fn(input).unwrap();
        let res = postfix_eval(&postfix, 3.).unwrap();
        assert_eq!(res, 54.);
    })
}

benchmark_group!(benches, a,);
benchmark_main!(benches);
