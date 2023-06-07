use {
    criterion::{
        criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
        PlottingBackend, Throughput,
    },
    std::{cmp, time::Duration},
};

mod data {
    pub const RUST_LIBRARY: &str = include_str!("data/rust-library.rs");

    pub mod pathological {
        pub const RANDOM_HUGE: &str = include_str!("data/pathological/random-huge.txt");

        pub const REPEATED_RARE_HUGE: &str =
            include_str!("data/pathological/repeated-rare-huge.txt");
        pub const REPEATED_RARE_SMALL: &str =
            include_str!("data/pathological/repeated-rare-small.txt");
    }

    pub mod sherlock {
        pub const TINY: &str =
            "Mr. Sherlock Holmes, who was usually very late in the mornings, save";
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Input {
    /// A name describing the corpus, used to identify it in benchmarks.
    pub name: &'static str,
    /// The haystack to search.
    pub corpus: &'static str,
    /// Queries that are expected to never occur.
    pub never: &'static [Query],
    /// Queries that are expected to occur rarely.
    pub rare: &'static [Query],
    /// Queries that are expected to fairly common.
    pub common: &'static [Query],
}

/// A substring search query for a particular haystack.
#[derive(Clone, Copy, Debug)]
pub struct Query {
    /// A name for this query, used to identify it in benchmarks.
    pub name: &'static str,
    /// The needle to search for.
    pub needle: &'static str,
    /// The expected number of occurrences.
    pub count: usize,
}

fn memmem(mut haystack: &[u8], needle: &[u8], find: fn(&[u8], &[u8]) -> Option<usize>) -> usize {
    let mut count = 0;
    while let Some(pos) = find(haystack, needle) {
        haystack = haystack.split_at(pos + cmp::max(1, needle.len())).1;
        count += 1;
    }
    count
}

fn define<M: Measurement>(
    group: &mut BenchmarkGroup<M>,
    (imp, corpus): (&str, &str),
    query: &Query,
    find: fn(&[u8], &[u8]) -> Option<usize>,
) {
    let &Query { needle, count, .. } = query;

    group.bench_function(imp, |b| {
        b.iter(|| assert_eq!(count, memmem(corpus.as_bytes(), needle.as_bytes(), find)))
    });
}

fn all_input(
    c: &mut Criterion,
    freq: &str,
    (name, corpus): (&str, &str), // => (name, haystack)
    query: &Query,
) {
    let mut group = c.benchmark_group(format!("{freq}/{name}/{}", query.name));
    group
        .throughput(Throughput::BytesDecimal(corpus.len() as u64))
        .sample_size(10)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2));

    use memmem::*;

    define(&mut group, ("naive", corpus), query, naive::find);
    define(&mut group, ("kmp", corpus), query, kmp::find);
    define(&mut group, ("kmp-optimized", corpus), query, kmp_optimized::find);

    if corpus.len() < data::RUST_LIBRARY.len() {
        define(&mut group, ("bm", corpus), query, bm::find);
    } else {
        println!("skip `bm` as so large input");
    }
    define(&mut group, ("bm-optimized", corpus), query, bm_optimized::find);
    // paste new gpt impls here

    define(&mut group, ("memchr", corpus), query, memchr::memmem::find);
}

fn all(c: &mut Criterion) {
    for input in INPUTS {
        for query in input.never {
            all_input(c, "never", (input.name, input.corpus), query);
        }
        for query in input.rare {
            all_input(c, "rare", (input.name, input.corpus), query);
        }
        for query in input.common {
            all_input(c, "common", (input.name, input.corpus), query);
        }
    }
}

pub const INPUTS: &[Input] = &[
    Input {
        name: "code-rust-library",
        corpus: data::RUST_LIBRARY,
        never: &[
            Query { name: "fn-strength", needle: "fn strength", count: 0 },
            Query { name: "fn-strength-paren", needle: "fn strength(", count: 0 },
            Query { name: "fn-quux", needle: "fn quux(", count: 0 },
        ],
        rare: &[Query { name: "fn-from-str", needle: "pub fn from_str(", count: 1 }],
        common: &[
            Query { name: "fn-is-empty", needle: "fn is_empty(", count: 17 },
            Query { name: "fn", needle: "fn", count: 2985 },
            Query { name: "paren", needle: "(", count: 30193 },
            Query { name: "let", needle: "let", count: 4737 },
        ],
    },
    Input {
        name: "pathological-repeated-rare-huge",
        corpus: data::pathological::REPEATED_RARE_HUGE,
        never: &[Query { name: "tricky", needle: "abczdef", count: 0 }],
        rare: &[],
        common: &[Query { name: "match", needle: "zzzzzzzzzz", count: 50010 }],
    },
    Input {
        name: "pathological-repeated-rare-small",
        corpus: data::pathological::REPEATED_RARE_SMALL,
        never: &[Query { name: "tricky", needle: "abczdef", count: 0 }],
        rare: &[],
        common: &[Query { name: "match", needle: "zzzzzzzzzz", count: 100 }],
    },
];

criterion_group!(
    name = benches;
    config = Criterion::default().plotting_backend(PlottingBackend::Plotters);
    targets = all
);
criterion_main!(benches);
