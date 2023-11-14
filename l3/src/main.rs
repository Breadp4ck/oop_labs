trait Operation<T> {
    fn name(&self) -> &'static str;
    fn designation(&self) -> &'static str;
    fn calclate(&self, a: T, b: T) -> T;
}

#[derive(Default)]
struct Xor;

impl Operation<u8> for Xor {
    fn name(&self) -> &'static str {
        "xor"
    }

    fn designation(&self) -> &'static str {
        "^"
    }

    fn calclate(&self, a: u8, b: u8) -> u8 {
        a ^ b
    }
}

#[derive(Default)]
struct Or;

impl Operation<u8> for Or {
    fn name(&self) -> &'static str {
        "or"
    }

    fn designation(&self) -> &'static str {
        "|"
    }

    fn calclate(&self, a: u8, b: u8) -> u8 {
        a | b
    }
}

#[derive(Default)]
struct And;

impl Operation<u8> for And {
    fn name(&self) -> &'static str {
        "and"
    }

    fn designation(&self) -> &'static str {
        "&"
    }

    fn calclate(&self, a: u8, b: u8) -> u8 {
        a & b
    }
}

#[derive(Default)]
struct Nand;

impl Operation<u8> for Nand {
    fn name(&self) -> &'static str {
        "nand"
    }

    fn designation(&self) -> &'static str {
        "↑"
    }

    fn calclate(&self, a: u8, b: u8) -> u8 {
        !(a & b)
    }
}

#[derive(Default)]
struct Nor;

impl Operation<u8> for Nor {
    fn name(&self) -> &'static str {
        "nor"
    }

    fn designation(&self) -> &'static str {
        "↓"
    }

    fn calclate(&self, a: u8, b: u8) -> u8 {
        !(a | b)
    }
}

fn main() {
    // Box это ссылка на память в куче. В Rust нельзя создавать объекты, реализующие типаж (trait)
    // на стеке, потому что размер таких объектов неизвестен на этапе компиляции.
    let operations: &[Box<dyn Operation<u8>>] = &[
        Box::new(Xor::default()),
        Box::new(And::default()),
        Box::new(Or::default()),
        Box::new(Nand::default()),
        Box::new(Nor::default()),
    ];

    let a = 0b1100;
    let b = 0b1010;

    for op in operations {
        println!(
            "{} {} {} = {} ({:08b} {} {:08b} = {:08b}) <-- {}",
            a,
            op.designation(),
            b,
            op.calclate(a, b),
            a,
            op.designation(),
            b,
            op.calclate(a, b),
            op.name(),
        );
    }
}
