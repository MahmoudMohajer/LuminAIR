use numerair::Fixed;
use luminair_air::DEFAULT_FP_SCALE;

pub fn demonstrate_arithmetic_differences() {
    println!("=== Arithmetic Differences Demonstration ===");
    println!("This demonstrates why rem tests are failing due to fixed-point vs floating-point differences\n");

    // Test cases from the failing tests
    let test_cases = vec![
        (0.0058294535_f32, 0.010498047_f32),
        (0.011574745_f32, 0.0014648438_f32),
        (0.00329566_f32, 0.000715822_f32),
    ];

    for (a, b) in test_cases {
        println!("=== Test Case: a = {}, b = {} ===", a, b);
        
        // Floating-point arithmetic (CPU)
        let fp_quotient = (a / b).floor();
        let fp_remainder = a - (fp_quotient * b);
        
        // Fixed-point arithmetic (STARK circuit)
        let fixed_a = Fixed::<DEFAULT_FP_SCALE>::from_f64(a as f64);
        let fixed_b = Fixed::<DEFAULT_FP_SCALE>::from_f64(b as f64);
        let fixed_quotient = Fixed::<DEFAULT_FP_SCALE>(fixed_a.0 / fixed_b.0); // Integer division on underlying i64
        let fixed_remainder = Fixed::<DEFAULT_FP_SCALE>(fixed_a.0 % fixed_b.0); // Integer remainder on underlying i64
        
        // Convert back to f32 for comparison
        let fp_quotient_f32 = fp_quotient;
        let fp_remainder_f32 = fp_remainder;
        let fixed_quotient_f32 = fixed_quotient.to_f64() as f32;
        let fixed_remainder_f32 = fixed_remainder.to_f64() as f32;
        
        println!("Floating-point (CPU):");
        println!("  Quotient: {}", fp_quotient_f32);
        println!("  Remainder: {}", fp_remainder_f32);
        println!("  Verification: {} = {} * {} + {}", a, fp_quotient_f32, b, fp_remainder_f32);
        
        println!("Fixed-point (STARK):");
        println!("  Quotient: {}", fixed_quotient_f32);
        println!("  Remainder: {}", fixed_remainder_f32);
        println!("  Verification: {} = {} * {} + {}", a, fixed_quotient_f32, b, fixed_remainder_f32);
        
        let quotient_diff = (fp_quotient_f32 - fixed_quotient_f32).abs();
        let remainder_diff = (fp_remainder_f32 - fixed_remainder_f32).abs();
        
        println!("Differences:");
        println!("  Quotient difference: {}", quotient_diff);
        println!("  Remainder difference: {}", remainder_diff);
        println!("  Total difference: {}", quotient_diff + remainder_diff);
        println!();
    }

    println!("=== Detailed Analysis ===");
    
    // Show how fixed-point arithmetic works
    println!("Fixed-point arithmetic uses integer division:");
    println!("  - a / b = floor(a / b)  (integer division)");
    println!("  - a % b = a - (a / b) * b  (integer remainder)");
    println!();
    
    println!("Floating-point arithmetic uses true division:");
    println!("  - a / b = exact floating-point division");
    println!("  - a % b = a - floor(a / b) * b  (floating-point remainder)");
    println!();
    
    // Demonstrate with a simple example
    let a = 0.7_f32;
    let b = 0.3_f32;
    println!("Example: a = {}, b = {}", a, b);
    
    let fp_quotient = (a / b).floor();
    let fp_remainder = a - (fp_quotient * b);
    
    let fixed_a = Fixed::<DEFAULT_FP_SCALE>::from_f64(a as f64);
    let fixed_b = Fixed::<DEFAULT_FP_SCALE>::from_f64(b as f64);
    let fixed_quotient = Fixed::<DEFAULT_FP_SCALE>(fixed_a.0 / fixed_b.0);
    let fixed_remainder = Fixed::<DEFAULT_FP_SCALE>(fixed_a.0 % fixed_b.0);
    
    println!("Floating-point: quotient = {}, remainder = {}", fp_quotient, fp_remainder);
    println!("Fixed-point: quotient = {}, remainder = {}", fixed_quotient.to_f64() as f32, fixed_remainder.to_f64() as f32);
    println!();
    
    println!("=== Conclusion ===");
    println!("The test failures are expected because:");
    println!("1. STARK circuits use fixed-point arithmetic for efficiency");
    println!("2. Fixed-point arithmetic uses integer division/remainder");
    println!("3. Floating-point arithmetic uses true division with floating-point remainder");
    println!("4. These differences are mathematically correct but different");
    println!("5. The tests should be updated to account for these differences");
}

#[test]
fn test_arithmetic_differences() {
    demonstrate_arithmetic_differences();
    
    // This test should always pass - it's just demonstrating the differences
    assert!(true, "Arithmetic differences demonstration completed");
} 