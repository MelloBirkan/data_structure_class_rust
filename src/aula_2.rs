struct PesoAltura {
    peso: i32,
    altura: i32,
}

impl PesoAltura {
    fn new(peso: i32, altura: i32) -> Self {
        Self { peso, altura }
    }
}

// const ALTURA_MAXIMA: u8 = 225;
pub fn main() {
    let fabio = PesoAltura {
        peso: 80,
        altura: 180
    };
    let chris = PesoAltura::new(98, 210);
    println!("{} e {} sao pessoas com o peso {} e {}",fabio.peso, chris.peso, fabio.altura, chris.altura );

    let mut x = 25;
    let y= &mut x;
    *y = 40;
    println!("O valor de x é {}", *y);
}
