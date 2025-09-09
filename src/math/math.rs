fn _norm_mod(x : i128, m : i128) -> i128 {
    let mut r = x % m;
    if r < 0 { r += m; }
    r
}

fn _mul_mod(a: i128, b: i128, m: i128) -> i128 {
    let mut res = (a * b) % m;
    if res < 0 { res += m;}
    res
}

fn  _mul_mod_2(a:i128, b:i128, m:i128) -> i128 {
    let a = ((a % m) + m) % m;
    let b = ((b % m) + m) % m;
    let res = (a * b) % m;
    res
}

fn _pow_mod(mut a:i128, mut e: u128, m:i128) -> i128 {
    if m <= 1 { return  0;}
    a = _norm_mod(a, m);
    let mut res : i128 = 1 % m;
    while e > 0 {
        if (e & 1) == 1 { res = _mul_mod(res, a, m)}
        a = _mul_mod(a, a, m);
        e >>= 1;
    }
    res
}

fn _egcd(a:i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        let g = if a > 0 { a} else {-a};
        let x = if a > 0 { 1} else { -1};
        let y = 0;
        return (g, x, y);
    }
    let (g, x1, y1)  = _egcd(b, a % b);
    (g, y1, (x1 - (a / b) * y1))
}

fn _inv_egcd(a:i128, m:i128) -> Option<i128> {
    if m <= 1 { return None;}
    let (g, x, _) = _egcd(a, m);
    if g != 1 && g != -1 { return None;}
    Some(_norm_mod(x, m))
}

fn _inv_fermat(a:i128, m:i128) -> Option<i128> {
    if m <= 1 && (a % m == 0) { return None;}
    Some(_pow_mod(a, (m-2) as u128, m))
}