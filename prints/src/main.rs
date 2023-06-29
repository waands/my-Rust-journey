enum FormaGeometrica {
    Retangulo { largura: u32, altura: u32 },
    Circulo(f64),
}

impl FormaGeometrica {
    fn calcular_area(&self) -> f64 {
        match self {
            FormaGeometrica::Retangulo { largura, altura } => f64::from(*largura * *altura),
            FormaGeometrica::Circulo(raio) => std::f64::consts::PI * raio.powi(2),
        }
    }
}

fn main() {
    let retangulo = FormaGeometrica::Retangulo {
        largura: 10,
        altura: 5,
    };
    let circulo = FormaGeometrica::Circulo(3.5);

    println!("Área do retângulo: {:.2}", retangulo.calcular_area());
    println!("Área do círculo: {:.2}", circulo.calcular_area());
}
