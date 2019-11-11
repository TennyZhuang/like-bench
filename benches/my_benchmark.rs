use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use like_bench::{like, like_optimize};

static cases: &'static [(&str, &str, char, std::option::Option<i64>)] = &[
    (r#"hello"#, r#"%HELLO%"#, '\\', Some(0)),
    (r#"Hello, World"#, r#"Hello, World"#, '\\', Some(1)),
    (r#"Hello, World"#, r#"Hello, %"#, '\\', Some(1)),
    (r#"Hello, World"#, r#"%, World"#, '\\', Some(1)),
    (r#"test"#, r#"te%st"#, '\\', Some(1)),
    (r#"test"#, r#"te%%st"#, '\\', Some(1)),
    (r#"test"#, r#"test%"#, '\\', Some(1)),
    (r#"test"#, r#"%test%"#, '\\', Some(1)),
    (r#"test"#, r#"t%e%s%t"#, '\\', Some(1)),
    (r#"test"#, r#"_%_%_%_"#, '\\', Some(1)),
    (r#"test"#, r#"_%_%st"#, '\\', Some(1)),
    (r#"C:"#, r#"%\"#, '\\', Some(0)),
    (r#"C:\"#, r#"%\"#, '\\', Some(1)),
    (r#"C:\Programs"#, r#"%\"#, '\\', Some(0)),
    (r#"C:\Programs\"#, r#"%\"#, '\\', Some(1)),
    (r#"C:"#, r#"%\\"#, '\\', Some(0)),
    (r#"C:\"#, r#"%\\"#, '\\', Some(1)),
    (r#"C:\Programs"#, r#"%\\"#, '\\', Some(0)),
    (r#"C:\Programs\"#, r#"%\\"#, '\\', Some(1)),
    (r#"C:\Programs\"#, r#"%Prog%"#, '\\', Some(1)),
    (r#"C:\Programs\"#, r#"%Pr_g%"#, '\\', Some(1)),
    (r#"C:\Programs\"#, r#"%%\"#, '%', Some(1)),
    (r#"C:\Programs%"#, r#"%%%"#, '%', Some(1)),
    (r#"C:\Programs%"#, r#"%%%%"#, '%', Some(1)),
    (r#"hello"#, r#"\%"#, '\\', Some(0)),
    (r#"%"#, r#"\%"#, '\\', Some(1)),
    (r#"3hello"#, r#"%%hello"#, '%', Some(1)),
    (r#"3hello"#, r#"3%hello"#, '3', Some(0)),
    (r#"3hello"#, r#"__hello"#, '_', Some(0)),
    (r#"3hello"#, r#"%_hello"#, '%', Some(1)),
    // special case
    (
        r#"aaaaaaaaaaaaaaaaaaaaaaaaaaa"#,
        r#"a%a%a%a%a%a%a%a%b"#,
        '\\',
        Some(0),
    ),
];

pub fn criterion_benchmark(c: &mut Criterion) {
    // for (i, (target, pattern, escape, _)) in cases.iter().enumerate() {
    //     c.bench_function(&format!("like {}", i), |b| b.iter(|| like(target.as_bytes(), pattern.as_bytes(), *escape as u32, 1).unwrap() as i64));
    //     c.bench_function(&format!("like_optimize {}", i), |b| b.iter(|| like_optimize(target.as_bytes(), pattern.as_bytes(), *escape as u32).unwrap() as i64));
    // }

    {
        let mut group = c.benchmark_group("attack_optimize");
        let target = "a".repeat(100);

        for i in 5..20 {
            group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
                let pattern = "a%".repeat(i) + ("b");
                b.iter(|| {
                    like_optimize(target.as_bytes(), pattern.as_bytes(), b'\\' as u32).unwrap()
                        as i64
                });
            });
        }
    }

    {
        let mut group = c.benchmark_group("attack");
        group.sample_size(10);
        let target = "a".repeat(100);

        for i in 1..7 {
            group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
                let pattern = "a%".repeat(i) + ("b");
                b.iter(|| {
                    like(target.as_bytes(), pattern.as_bytes(), b'\\' as u32, 1).unwrap() as i64
                });
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
