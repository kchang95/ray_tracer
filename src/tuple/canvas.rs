use crate::tuple::tuple::Tuple;
use array2d::{Array2D, Error};
use std::convert::TryFrom;

struct Canvas {
    width: usize,
    height: usize,
    data: Array2D<Tuple>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let (width, height) = Canvas::u32_to_usize(width, height);
        let create_tuple = || Tuple::color(0.0, 0.0, 0.0);
        let new_array = Array2D::filled_with(create_tuple(), height, width);
        Canvas {
            width,
            height,
            data: new_array,
        }
    }

    pub fn u32_to_usize(width: u32, height: u32) -> (usize, usize) {
        (
            usize::try_from(width).unwrap(),
            usize::try_from(height).unwrap(),
        )
    }

    pub fn update_pixel(&mut self, width: u32, height: u32, new_point: Tuple) -> Result<(), Error> {
        let (width, height) = Canvas::u32_to_usize(width, height);
        self.data.set(height, width, new_point)?;
        Ok(())
    }

    fn generate_ppm_header(&self, max_value: u32) -> String {
        format!("P3\n{} {}\n{}", self.width, self.height, max_value)
    }

    fn convert_to_ppm(&self, max_value: u32) -> String {
        let mut result = String::from("");
        let array_iter = self.data.elements_row_major_iter();
        let mut char_count = 0;
        for pixel in array_iter {
            for pixel_element in vec![pixel.x, pixel.y, pixel.z].iter() {
                let mut new_pixel_element = *pixel_element;
                if new_pixel_element > 1.0 {
                    new_pixel_element = max_value as f64
                } else if new_pixel_element < 0.0 {
                    new_pixel_element = 0.0
                } else {
                    new_pixel_element = new_pixel_element * max_value as f64
                };

                let mut new_element = new_pixel_element.to_string();
                let new_element_char_count = new_element.chars().count();
                if char_count + new_element_char_count + 1 > 70 {
                    new_element = format!("\n{}", new_element);
                    char_count = new_element_char_count;
                } else {
                    new_element = format!(" {}", new_element);
                    char_count = char_count + new_element_char_count + 1;
                }

                result = format!("{}{}", result, new_element);
            }
        }
        result.remove(0).to_string();
        result
    }

    fn max(&self) -> (f64, f64, f64) {
        let (mut max_r, mut max_g, mut max_b) = (0.0, 0.0, 0.0);
        let find_max = |element: &Tuple| {
            if element.x > max_r {
                max_r = element.x
            }
            if element.y > max_g {
                max_g = element.y
            }
            if element.z > max_b {
                max_b = element.z
            }
        };
        self.data.elements_row_major_iter().for_each(find_max);
        (max_r, max_g, max_b)
    }

    pub fn to_ppm(&self, max_value: u32) -> String {
        format!(
            "{}\n{}\n",
            self.generate_ppm_header(max_value),
            self.convert_to_ppm(max_value)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_new() {
        let canvas = Canvas::new(10, 20);
        let expected_canvas = Canvas {
            width: 10usize,
            height: 20usize,
            data: Array2D::filled_with(Tuple::color(0.0, 0.0, 0.0), 20, 10),
        };
        assert_eq!(canvas.width, expected_canvas.width);
        assert_eq!(canvas.height, expected_canvas.height);
        assert_eq!(canvas.data, expected_canvas.data);
    }

    #[test]
    fn test_update_pixel() {
        let mut new_canvas = Canvas::new(10, 20);
        let red = Tuple::color(1.0, 0.0, 0.0);
        assert_eq!(new_canvas.update_pixel(2, 3, red), Ok(()));

        let expected_element = Tuple::color(1.0, 0.0, 0.0);
        let element = new_canvas.data.get(3, 2).unwrap();
        assert_eq!(*element, expected_element)
    }

    #[test]
    fn test_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let expected_header = "P3\n5 3\n255".to_string();
        assert_eq!(canvas.generate_ppm_header(255), expected_header)
    }

    #[test]
    fn test_max() {
        let mut canvas = Canvas::new(5, 3);
        canvas.data.set(0, 0, Tuple::color(1.0, 0.0, 0.0)).unwrap();
        canvas.data.set(1, 2, Tuple::color(1.0, 0.5, 0.0)).unwrap();
        canvas.data.set(2, 4, Tuple::color(-1.0, 0.0, 2.0)).unwrap();

        let max = canvas.max();
        assert_eq!(max, (1.0, 0.5, 2.0));
    }

    #[test]
    fn test_convert_to_ppm() {
        let mut canvas = Canvas::new(5, 3);
        canvas.data.set(0, 0, Tuple::color(1.5, 0.0, 0.0)).unwrap();
        canvas.data.set(1, 2, Tuple::color(0.0, 0.5, 0.0)).unwrap();
        canvas.data.set(2, 4, Tuple::color(-0.5, 0.0, 1.0)).unwrap();

        let result = canvas.convert_to_ppm(255);
        let expected_result =
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127.5 0 0 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 0 0 0 0 0 255"
                .to_string();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_to_ppm() {
        let mut canvas = Canvas::new(5, 3);
        canvas.data.set(0, 0, Tuple::color(1.5, 0.0, 0.0)).unwrap();
        canvas.data.set(1, 2, Tuple::color(0.0, 0.5, 0.0)).unwrap();
        canvas.data.set(2, 4, Tuple::color(-0.5, 0.0, 1.0)).unwrap();

        let result = canvas.to_ppm(255);
        let expected_result =
            "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127.5 0 0 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 0 0 0 0 0 255\n"
                .to_string();
        assert_eq!(result, expected_result);
    }
}
