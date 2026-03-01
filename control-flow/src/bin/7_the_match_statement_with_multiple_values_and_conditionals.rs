fn main() {
    let number = 8;

    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        _ => unreachable!(), //jaroorat nahi agar default case ki toh ye use hoga.here you can see that both arm covers even and odd but there is no default case.so that why we must to add this line
    }
}
//upar hame kon hai woh dena hai(matlab ki jisse match karana hai woh.yaha pe jaise season)
//left side pe hame right vali value deni hai(it could be anything like cases)