use crate::utility;

#[derive(Debug, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1f64 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0f64 }
    }

    pub fn color(red: f64, green: f64, blue: f64) -> Tuple {
        Tuple {
            x: red,
            y: green,
            z: blue,
            w: 0f64,
        }
    }

    pub fn add(first: &Tuple, second: &Tuple) -> Tuple {
        Tuple {
            x: first.x + second.x,
            y: first.y + second.y,
            z: first.z + second.z,
            w: first.w + second.w,
        }
    }

    pub fn subtract(first: &Tuple, second: &Tuple) -> Tuple {
        Tuple {
            x: first.x - second.x,
            y: first.y - second.y,
            z: first.z - second.z,
            w: first.w - second.w,
        }
    }

    pub fn negate(tuple: &Tuple) -> Tuple {
        Tuple {
            x: -tuple.x,
            y: -tuple.y,
            z: -tuple.z,
            w: -tuple.w,
        }
    }

    pub fn scale(tuple: &Tuple, factor: f64) -> Tuple {
        Tuple {
            x: factor * tuple.x,
            y: factor * tuple.y,
            z: factor * tuple.z,
            w: factor * tuple.w,
        }
    }

    pub fn magnitude(&self) -> f64 {
        let square_sum = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        square_sum.sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot_product(first: &Tuple, second: &Tuple) -> f64 {
        first.x * second.x + first.y * second.y + first.z * second.z + first.w * second.w
    }

    pub fn cross_product(first: &Tuple, second: &Tuple) -> Tuple {
        Tuple {
            x: first.y * second.z - first.z * second.y,
            y: first.z * second.x - first.x * second.z,
            z: first.x * second.y - first.y * second.x,
            w: 0f64,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let equal = utility::equal_float(&self.x, &other.x);
        let equal = equal && utility::equal_float(&self.y, &other.y);
        let equal = equal && utility::equal_float(&self.z, &other.z);
        let equal = equal && utility::equal_float(&self.w, &other.w);
        equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let test_point = Tuple::point(0.0, 1.0, -1.0);
        let test_tuple = Tuple {
            x: 0.0,
            y: 1.0,
            z: -1.0,
            w: 1f64,
        };
        assert_eq!(test_point, test_tuple)
    }

    #[test]
    fn test_vector() {
        let test_vector = Tuple::vector(0.0, 1.0, -1.0);
        let test_tuple = Tuple {
            x: 0.0,
            y: 1.0,
            z: -1.0,
            w: 0f64,
        };
        assert_eq!(test_vector, test_tuple)
    }

    #[test]
    fn test_add_tuple() {
        let test_point = Tuple::point(3.0, -2.0, 5.0);
        let test_vector = Tuple::vector(-2.0, 3.0, 1.0);
        let result = Tuple::add(&test_point, &test_vector);
        let actual_result = Tuple::point(1.0, 1.0, 6.0);
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_subtract_points() {
        let test_point_1 = Tuple::point(3.0, 2.0, 1.0);
        let test_point_2 = Tuple::point(5.0, 6.0, 7.0);
        let result = Tuple::subtract(&test_point_1, &test_point_2);
        let actual_result = Tuple::vector(-2.0, -4.0, -6.0);
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_subtract_vector_point() {
        let test_point = Tuple::point(3.0, 2.0, 1.0);
        let test_vector = Tuple::vector(5.0, 6.0, 7.0);
        let result = Tuple::subtract(&test_point, &test_vector);
        let actual_result = Tuple::point(-2.0, -4.0, -6.0);
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_subtract_vectors() {
        let test_vector_1 = Tuple::vector(3.0, 2.0, 1.0);
        let test_vector_2 = Tuple::vector(5.0, 6.0, 7.0);
        let result = Tuple::subtract(&test_vector_1, &test_vector_2);
        let actual_result = Tuple::vector(-2.0, -4.0, -6.0);
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_negating_tuple() {
        let test_tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let result = Tuple::negate(&test_tuple);
        let actual_result = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_scale_tuple() {
        let test_tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let result = Tuple::scale(&test_tuple, 3.5);
        let actual_result = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_scale_tuple_fraction() {
        let test_tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let result = Tuple::scale(&test_tuple, 0.5);
        let actual_result = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_tuple_magnitude() {
        let test_tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let result = test_tuple.magnitude();
        let actual_result: f64 = 1.0 + 4.0 + 9.0 + 16.0;
        let actual_result = actual_result.sqrt();
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_vector_normalize() {
        let test_vector = Tuple::vector(1.0, 2.0, 3.0);
        let result = test_vector.normalize();
        let actual_result = test_vector.magnitude();
        let actual_result = Tuple::vector(
            1.0 / actual_result,
            2.0 / actual_result,
            3.0 / actual_result,
        );
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_vector_dot_product() {
        let test_vector_1 = Tuple::vector(1.0, 2.0, 3.0);
        let test_vector_2 = Tuple::vector(2.0, 3.0, 4.0);
        let result = Tuple::dot_product(&test_vector_1, &test_vector_2);
        let actual_result = 2.0 + 6.0 + 12.0;
        assert_eq!(result, actual_result)
    }

    #[test]
    fn test_vector_cross_product() {
        let test_vector_1 = Tuple::vector(1.0, 2.0, 3.0);
        let test_vector_2 = Tuple::vector(2.0, 3.0, 4.0);
        let result = Tuple::cross_product(&test_vector_1, &test_vector_2);
        let actual_result = Tuple::vector(-1.0, 2.0, -1.0);
        assert_eq!(result, actual_result);

        let result = Tuple::cross_product(&test_vector_2, &test_vector_1);
        let actual_result = Tuple::vector(1.0, -2.0, 1.0);
        assert_eq!(result, actual_result)
    }
}
