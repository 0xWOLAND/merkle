pub fn hello() -> String {
    String::from("u512 hello")
}

fn rotr(v: &u32, k: u32) -> u32 {
    (v >> k) | (v << (32 - k))
}

pub fn sigma_1(v: &u32) -> u32 {
    rotr(&v, 17) ^ rotr(&v, 19) ^ (v >> 10)
}

pub fn sigma_0(v: &u32) -> u32 {
    rotr(&v, 7) ^ rotr(&v, 18) ^ (v >> 3)
}

pub fn Sigma_0(v: &u32) -> u32 {
    rotr(&v, 2) ^ rotr(&v, 13) ^ rotr(&v, 22)
}

pub fn Sigma_1(v: &u32) -> u32 {
    rotr(&v, 6) ^ rotr(&v, 11) ^ rotr(&v, 25)
}

pub fn choose(e: &u32, f: &u32, g: &u32) -> u32 {
    (e & f) ^ (!e & g)
}

pub fn majority(a: &u32, b: &u32, c: &u32) -> u32 {
    (a & b) | (b & c) | (a & c)
}

pub fn T1(h: &u32, e: &u32, f: &u32, g: &u32, K_0: &u32, w_0: &u32) -> u32 {
    h.wrapping_add(Sigma_1(e))
        .wrapping_add(choose(e, f, g))
        .wrapping_add(*K_0)
        .wrapping_add(*w_0)
}

pub fn T2(a: &u32, b: &u32, c: &u32) -> u32 {
    Sigma_0(a).wrapping_add(majority(a, b, c))
}
