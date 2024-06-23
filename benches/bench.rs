use levenshtein_lite::LevenshteinAutomata;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Benchmarks for d=0
fn bench_instantiation_d0(c: &mut Criterion) {
    c.bench_function("instantiation_d0", |b| {
        b.iter(|| LevenshteinAutomata::new(black_box("Levenshtein"), black_box(0)));
    });
}

fn bench_check_d0_match(c: &mut Criterion) {
    let lda = LevenshteinAutomata::new("Levenshtein", 0);
    c.bench_function("check_d0_match", |b| {
        b.iter(|| lda.check(black_box("Levenshtein")));
    });
}

fn bench_check_d0_no_match(c: &mut Criterion) {
    let lda = LevenshteinAutomata::new("Levenshtein", 0);
    c.bench_function("check_d0_no_match", |b| {
        b.iter(|| lda.check(black_box("Levenshteinx")));
    });
}

// Benchmarks for d=1
fn bench_instantiation_d1(c: &mut Criterion) {
    c.bench_function("instantiation_d1", |b| {
        b.iter(|| LevenshteinAutomata::new(black_box("Levenshtein"), black_box(1)));
    });
}

fn bench_check_d1_match(c: &mut Criterion) {
    let lda = LevenshteinAutomata::new("Levenshtein", 1);
    c.bench_function("check_d1_match", |b| {
        b.iter(|| lda.check(black_box("Levenshtein")));
    });
}

fn bench_check_d1_no_match(c: &mut Criterion) {
    let lda = LevenshteinAutomata::new("Levenshtein", 1);
    c.bench_function("check_d1_no_match", |b| {
        b.iter(|| lda.check(black_box("Xevenshteinx")));
    });
}

// Benchmarks for d=2
fn bench_instantiation_d2(c: &mut Criterion) {
    c.bench_function("instantiation_d2", |b| {
        b.iter(|| LevenshteinAutomata::new(black_box("Levenshtein"), black_box(2)));
    });
}

fn bench_check_d2_match(c: &mut Criterion) {
    let lda = LevenshteinAutomata::new("Levenshtein", 2);
    c.bench_function("check_d2_match", |b| {
        b.iter(|| lda.check(black_box("Levenshtein")));
    });
}

fn bench_check_d2_no_match(c: &mut Criterion) {
    let lda = LevenshteinAutomata::new("Levenshtein", 2);
    c.bench_function("check_d2_no_match", |b| {
        b.iter(|| lda.check(black_box("Xxvenshteinx")));
    });
}

// Benchmarks for d=4
fn bench_instantiation_d4(c: &mut Criterion) {
    c.bench_function("instantiation_d4", |b| {
        b.iter(|| LevenshteinAutomata::new(black_box("Levenshtein"), black_box(4)));
    });
}

fn bench_check_d4_match(c: &mut Criterion) {
    let lda = LevenshteinAutomata::new("Levenshtein", 4);
    c.bench_function("check_d4_match", |b| {
        b.iter(|| lda.check(black_box("Levenshtein")));
    });
}

fn bench_check_d4_no_match(c: &mut Criterion) {
    let lda: LevenshteinAutomata = LevenshteinAutomata::new("Levenshtein", 4);
    c.bench_function("check_d4_no_match", |b| {
        b.iter(|| lda.check(black_box("Xxxxnshteinx")));
    });
}

criterion_group!(
    benches,
    bench_instantiation_d0,
    bench_check_d0_match,
    bench_check_d0_no_match,
    bench_instantiation_d1,
    bench_check_d1_match,
    bench_check_d1_no_match,
    bench_instantiation_d2,
    bench_check_d2_match,
    bench_check_d2_no_match,
    bench_instantiation_d4,
    bench_check_d4_match,
    bench_check_d4_no_match,
);
criterion_main!(benches);