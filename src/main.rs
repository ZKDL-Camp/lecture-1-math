pub mod group;

#[cfg(test)]
mod tests {
    use super::*;
    use group::*;
    use nalgebra::Rotation3;

    #[test]
    fn test_matrices_are_group() {
        assert_group_valid::<Rotation3<f32>>();
    }

    #[test]
    #[should_panic]
    fn test_matrices_are_not_abelian() {
        assert_group_abelian::<Rotation3<f32>>();
    }

    #[test]
    fn test_integers_are_group() {
        assert_group_valid::<i64>()
    }

    #[test]
    fn test_integers_are_abelian() {
        assert_group_abelian::<i64>();
    }
}

fn main() {}
