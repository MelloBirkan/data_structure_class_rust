const MAX: usize = 50;

type TIPOCHAVE = i32;

#[derive(Copy, Clone)]
struct Registro {
    chave: TIPOCHAVE,
}

struct Lista {
    a: [Registro; MAX] ,
    nro_elem: u32
}

impl Lista {
    fn new() -> Self {
        Self {
            a: [Registro {chave: 0}; MAX],
            nro_elem: 0
        }
    }

    fn nroElem(&self) -> u32 {
        self.nro_elem
    }

    fn print_lista(&self) {
        print!("Lista [");
        for chave in self.a {
            if chave.chave == 0 {
                continue
            } else {
                print!("{} ", chave.chave);
            }
        }
        print!("]");
    }
}

pub fn main() {
    let mut lista: Lista = Lista::new();
    lista.print_lista();
}