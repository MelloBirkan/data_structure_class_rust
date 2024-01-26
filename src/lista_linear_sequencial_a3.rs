const MAX: usize = 50;

type TIPOCHAVE = i32;

#[derive(Copy, Clone)]
struct Registro {
    chave: TIPOCHAVE,
}

struct Lista {
    a: [Registro; MAX],
    nro_elem: u32,
}

impl Lista {
    fn new() -> Self {
        Self {
            a: [Registro { chave: 0 }; MAX],
            nro_elem: 0,
        }
    }

    fn nro_elem(&self) -> u32 {
        self.nro_elem
    }

    fn print_lista(&self) {
        print!("Lista [");
        for valor in self.a {
            if valor.chave == 0 {
                continue;
            } else {
                print!("{} ", valor.chave);
            }
        }
        println!("]");
    }

    fn busca(&self, chave: TIPOCHAVE) -> Option<usize> {
        for (i, valor) in self.a.iter().enumerate() {
            if valor.chave == chave {
                return Some(i);
            }
        }
        None
    }

    fn add(&mut self, reg: Registro, posicao: usize) -> bool {
        if posicao >= MAX || self.nro_elem == MAX as u32 {
            return false;
        }
        let mut j: usize = self.nro_elem as usize;
        while j > posicao {
            self.a[j] = self.a[j - 1];
            j -= 1;
        }
        self.a[posicao] = reg;
        self.nro_elem += 1;
        true
    }

    fn del(&mut self, valor: TIPOCHAVE) -> bool {
        let valor_i = match self.busca(valor) {
            Some(valor) => valor,
            None => return false,
        };
        let mut j: usize = self.nro_elem as usize;
        while j > valor_i {
            self.a[j - 1] = self.a[j];
            j -= 1;
        }
        self.nro_elem -= 1;
        true
    }
}

pub fn main() {
    let mut lista: Lista = Lista::new();
    println!("{}", lista.add(Registro { chave: 12 }, 0));

    println!("{:?}", lista.del(12));
    println!("{}", lista.nro_elem());
    lista.print_lista();
}
