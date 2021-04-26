use rand::RngCore;

fn rng_is_hard(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    quad_rand::compat::QuadRand.fill_bytes(buf);
    Ok(())
}

getrandom::register_custom_getrandom!(rng_is_hard);
