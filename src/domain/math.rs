use actix_web::http::header::Range;

pub fn gcd(mut x: i32, mut y: i32) -> i32 {
    if y > x {
        let tmp = x;
        x = y;
        y = tmp;
    }
    let result: i32 = if x % y == 0 {
        y
    } else {
        gcd(y, x % y)
    };
    return result;
}

pub fn max(x: i32, y: i32) -> i32 {
    return if x > y {
        x
    } else {
        y
    };
}

pub fn min(x: i32, y: i32) -> i32 {
    return if x < y {
        x
    } else {
        y
    };
}

pub fn abs(x: i32) -> i32 {
    return if x < 0 {
        -x
    } else {
        x
    };
}

pub fn pow(x: i32, n: i32) -> i32 {
    if n < 0 {
        return 0;
    } else if n == 0 {
        return 1;
    }

    let mut result = x;
    let mut i = 2;
    while i <= n {
        result *= x;
        i += 1;
    }
    result
}