use zbase32::{decode, encode};

fn main() {
    divan::main();
}

/// Input sizes in bytes, covering short inputs (hashes, keys) up to larger payloads.
const SIZES: &[usize] = &[8, 32, 1024, 65536];

fn payload(size: usize) -> Vec<u8> {
    // Deterministic pseudo-random bytes so results are stable across runs.
    let mut state = 0x2545_f491_4f6c_dd1du64;
    (0..size)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 24) as u8
        })
        .collect()
}

#[divan::bench(args = SIZES)]
fn encode_bytes(bencher: divan::Bencher, size: usize) {
    let input = payload(size);
    bencher.bench_local(|| divan::black_box(encode(divan::black_box(&input))));
}

#[divan::bench(args = SIZES)]
fn decode_str(bencher: divan::Bencher, size: usize) {
    let encoded = encode(payload(size));
    bencher.bench_local(|| divan::black_box(decode(divan::black_box(encoded.as_str()))));
}

#[divan::bench(args = SIZES)]
fn roundtrip(bencher: divan::Bencher, size: usize) {
    let input = payload(size);
    bencher.bench_local(|| divan::black_box(decode(&encode(divan::black_box(&input)))));
}

#[divan::bench]
fn decode_invalid() {
    // Error path: an invalid character is hit at the very end of the input.
    let mut input = encode(payload(1024));
    input.push('#');
    divan::black_box(decode(divan::black_box(input.as_str()))).ok();
}
