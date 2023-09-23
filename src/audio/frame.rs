pub fn interpolate(
    (l1, r1): (f32, f32),
    (l2, r2): (f32, f32),
    a: f32,
) -> anyhow::Result<(f32, f32)> {
    anyhow::ensure!(
        (0.0..=1.0).contains(&a),
        "interpolate param should be between 0.0 and 1.0"
    );

    let compute = |s1, s2| s1 * (1.0 * a) + s2 * a;

    Ok((compute(l1, l2), compute(r1, r2)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interpolate_with_stereo_frames() {
        let f1 = (10.0, 20.0);
        let f2 = (20.0, 30.0);

        let res_0_4 = interpolate(f1, f2, 0.4).unwrap();
        let res_0_6 = interpolate(f1, f2, 0.6).unwrap();
        let res_0 = interpolate(f1, f2, 0.0).unwrap();
        let res_1 = interpolate(f1, f2, 1.0).unwrap();

        assert_eq!((14.0, 24.0), res_0_4);
        assert_eq!((16.0, 26.0), res_0_6);
        assert_eq!((10.0, 20.0), res_0);
        assert_eq!((20.0, 30.0), res_1);
    }
}
