//! Optimized computations - Faster than Excel

/// Check if CPU supports AVX (Advanced Vector Extensions)
pub fn has_avx() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // We'll implement this later
        false
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Optimized sum using chunk processing (manual SIMD-like optimization)
pub fn optimized_sum(data: &[f64]) -> f64 {
    if data.len() < 8 {
        return data.iter().sum();
    }
    
    let mut sum = 0.0;
    let mut chunks = data.chunks_exact(8);
    
    // Process 8 elements at a time (like SIMD would)
    for chunk in chunks.by_ref() {
        // Manual unrolling - compiler may optimize this to SIMD
        sum += chunk[0] + chunk[1] + chunk[2] + chunk[3] +
               chunk[4] + chunk[5] + chunk[6] + chunk[7];
    }
    
    // Add any remaining elements
    sum + chunks.remainder().iter().sum::<f64>()
}

/// Compare optimized vs scalar performance
pub fn benchmark_sum(data: &[f64]) -> (f64, f64, f64) {
    use std::time::Instant;
    
    // Time scalar sum (like Excel)
    let start = Instant::now();
    let scalar_result: f64 = data.iter().sum();
    let scalar_time = start.elapsed().as_nanos();
    
    // Time optimized sum (our version)
    let start = Instant::now();
    let optimized_result = optimized_sum(data);
    let optimized_time = start.elapsed().as_nanos();
    
    let speedup = if optimized_time > 0 {
        scalar_time as f64 / optimized_time as f64
    } else {
        1.0
    };
    
    (scalar_result, optimized_result, speedup)
}

/// Calculate multiple statistics at once (more efficient than separate calls)
pub fn calculate_stats(data: &[f64]) -> (f64, f64, f64, f64, f64) {
    if data.is_empty() {
        return (0.0, 0.0, 0.0, 0.0, 0.0);
    }
    
    let mut sum = 0.0;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    
    // Single pass through data - more efficient than separate min/max/sum calls
    for &value in data {
        sum += value;
        min = min.min(value);
        max = max.max(value);
    }
    
    let count = data.len() as f64;
    let average = sum / count;
    
    (sum, average, min, max, count)
}