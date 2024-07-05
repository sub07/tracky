use eyre::ensure;

#[inline]
pub fn linear_f32((l1, r1): (f32, f32), (l2, r2): (f32, f32), a: f32) -> eyre::Result<(f32, f32)> {
    ensure!(
        (0.0..=1.0).contains(&a),
        "interpolate param should be between 0.0 and 1.0"
    );

    #[inline]
    fn compute(s1: f32, s2: f32, a: f32) -> f32 {
        s1 * (1.0 - a) + s2 * a
    }

    Ok((compute(l1, l2, a), compute(r1, r2, a)))
}
