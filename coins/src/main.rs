enum Barya {
    Centavo,
    Piso,
    LimangPiso,
    SampungPiso
}

fn value(barya: Barya) -> f32 {
    match barya {
        Barya::Centavo => 0.25,
        Barya::Piso => 1.0,
        Barya::LimangPiso => 5.0,
        Barya::SampungPiso => 10.0
    }
}
fn main() {
    let barya = Barya::SampungPiso;

    println!("Meron akong sampung piso : P{}", value(barya));
}
