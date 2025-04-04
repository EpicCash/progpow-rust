extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate num_bigint; // Replace bigint with num_bigint
extern crate dirs;

//extern crate progpow_cpu;

#[cfg(feature = "cuda")]
extern crate progpow_gpu_cuda as progpow_gpu;
#[cfg(feature = "opencl")]
extern crate progpow_gpu_opencl as progpow_gpu;

pub mod hardware;
pub mod types;

#[cfg(test)]
mod test {
    use super::*;

    use num_bigint::BigUint; // Import BigUint from num_bigint
    use num_traits::{One, Zero}; // For utility methods like max_value and division
    use hardware::PpCPU;
    use types::PpCompute;

    #[test]
    fn test_compute_cpu() {
        let height: u64 = 20;
        let nonce: u64 = 10123012301;
        let header_hash: [u8; 32] = [0; 32];
        let pp_cpu = PpCPU::new();
        let (value, mix) = pp_cpu.verify(&header_hash, height, nonce).unwrap();
        assert_eq!(
            mix,
            [
                1067276040, 109748694, 1270962088, 3616890847, 2528371908, 2524623649, 1191460869,
                2529877558
            ]
        );
    }

    #[test]
    #[cfg(feature = "gpu")]
    fn test_compute_gpu() {
        use hardware::PpGPU;
        use progpow_gpu::utils::get_gpu_solution;

        let header = [20u8; 32];
        let epoch: i32 = 0;
        let height: u64 = 1;
        let boundary: u64 = 100000000;

        let mut difficulty = BigUint::one() << 256; // Equivalent to U256::max_value()
        difficulty = difficulty / BigUint::from(boundary);
        let target = difficulty >> 192;

        let (nonce, mix) = get_gpu_solution(header.clone(), height, epoch, boundary);
        let cpu = PpCPU::new();
        let (value, mix_hash) = cpu.verify(&header, height, nonce).unwrap();

        let mix32: [u32; 8] = unsafe { ::std::mem::transmute(mix) };

        assert_eq!(mix32, mix_hash);
        assert!(((value[0] as u64) << 32 | value[1] as u64) < target.to_u64_digits()[0]);
    }
}
