fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn transform(subject: u128, loop_size: u128) -> u128 {
    mod_pow(subject, loop_size, 20201227)
}
fn public_key(loop_size: u128) -> u128 {
    transform(7, loop_size)
}
fn figure_out_loop(pkey: u128) -> u128 {
    (0..1000000000)
        .skip_while(|v| transform(7, *v) != pkey)
        .nth(0)
        .unwrap()
}
fn main() {
    // 11562782 real
    let card_pkey = 11562782;
    let door_pkey = 18108497;
    // real 18108497
    let door_loop = figure_out_loop(door_pkey);
    assert_eq!(door_pkey, public_key(door_loop));
    dbg!(door_loop);
    // Figure out card size
    let card_loop = figure_out_loop(card_pkey);
    assert_eq!(card_pkey, public_key(card_loop));
    dbg!(card_loop);

    // Figure out card size

    let encryption = transform(card_pkey, door_loop);
    dbg!(encryption);
}
