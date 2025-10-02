use luminair::prelude::*;

/// Test different fixed-point scales to verify dynamic scaling works
fn test_scale(scale: u32, test_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 Testing scale = {} ({})", scale, test_name);
    
    let mut cx = Graph::new();

    // Create simple computation: a * b + c
    let a = cx.tensor((2, 2)).set(vec![1.5, 2.7, 3.1, 4.9]);
    let b = cx.tensor((2, 2)).set(vec![10.0, 20.0, 30.0, 40.0]);
    let c = cx.tensor((2, 2)).set(vec![0.5, 1.0, 1.5, 2.0]);
    
    let result = a * b + c;
    let mut result = result.retrieve();

    // Compile with the specified scale
    cx.compile(<(GenericCompiler, StwoCompiler)>::default(), &mut result);

    // Generate circuit settings with dynamic scale
    let mut settings = cx.gen_circuit_settings(scale);
    
    // Verify the scale was actually set
    assert_eq!(settings.fixed_point_scale, scale);
    println!("✅ Scale {} correctly set in settings", settings.fixed_point_scale);

    // Generate trace and proof
    let trace = cx.gen_trace(&mut settings)?;
    println!("✅ Trace generated successfully with scale {}", scale);
    
    let proof = prove(trace, settings.clone())?;
    println!("✅ Proof generated successfully with scale {}", scale);
    
    // Verify proof
    verify(proof, settings)?;
    println!("✅ Proof verification successful with scale {}", scale);
    
    // Show final result
    println!("Final result: {:?}", result);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 RIGOROUS TESTING: Dynamic Fixed-Point Scale Implementation");
    println!("=" .repeat(60));

    // Test scales: 4, 8, 16, 32 (different precision levels)
    test_scale(4, "Low precision")?;
    test_scale(8, "Medium precision")?;
    test_scale(16, "High precision")?;
    test_scale(32, "Very high precision")?;
    
    println!("\n🎉 ALL SCALE TESTS PASSED! Dynamic scaling works correctly!");
    
    Ok(())
}
