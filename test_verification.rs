fn main() {
    use luminair::*;
    let air = simple();
    let (trace, settings) = air.gen_trace(12)?;
    println!("Settings fixed_point_scale: {}", settings.fixed_point_scale);
    println!("Verifying proof...");
    let proof = prove(trace.clone(), settings.clone())?;
    println!("Verifying...");
    verify(proof, settings)?;
    println!("Complete!");
}
