pub mod grapher {
    use std::convert::From;
    use std::f64;
    use std::fmt::Debug;
    use std::fmt::Display;

    fn scale_y_value(
        values: &[f64],
        new_min: i16,
        new_max: i16,
        scale_from_old_zero: bool,
    ) -> Vec<f64> {
        let mut scaled_value: Vec<f64> = Vec::new();
        let (mut y_min_value, y_max_value) = get_min_max(&values);
        if scale_from_old_zero {
            y_min_value = 0.0;
        }
        //let new_min = 0;
        let mut old_range = y_max_value - y_min_value;
        if old_range == 0.0 {
            old_range = 1.0;
        };
        let new_range: f64 = (new_max - new_min) as f64;
        for i in values {
            scaled_value.push((((i - y_min_value) * new_range) / old_range) + new_min as f64);
        }
        scaled_value.to_owned()
    }

    fn get_ascii_field(values: Vec<f64>) -> Vec<Vec<char>> {
        let _empty_space = ' ';
        let mut field: Vec<Vec<char>> = vec![];
        for _i in 0..(values.len() as i32) {
            let mut temp: Vec<char> = Vec::new();
            for _j in 0..(values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)) as i32 + 1) {
                temp.push(_empty_space)
            }
            field.push(temp);
        }
        for x in 0..(values.len()) as usize {
            let y = values[x];
            let y_prev = if x != 0 { values[x - 1] } else { y };
            let y_next = if x != values.len() - 1 {
                values[x + 1]
            } else {
                y
            };
            if (y_prev - y) > 1_f64 || y_prev - y < -1_f64 {
                let step = if y_prev - y > 0.0 { 1 } else { -1 };
                let mut _h = y as i32 + step;
                if step < 0 {
                    while _h > y_prev as i32 {
                        if field[x as usize][_h as usize] == _empty_space {
                            field[x as usize][_h as usize] = '|';
                        }
                        _h = _h + step;
                    }
                } else {
                    while _h < y_prev as i32 {
                        if field[x as usize][_h as usize] == _empty_space {
                            field[x as usize][_h as usize] = '|';
                        }
                        _h = _h + step;
                    }
                }
                for _i in ((y as i32 + step)..y_prev as i32).step_by(step as usize) {
                    if field[x as usize][_i as usize] == _empty_space {
                        field[x as usize][_i as usize] = '|'
                    }
                }
            }
            field[x as usize][y as usize] = get_ascii_char(y_prev, y, y_next)
        }
        field.to_owned()
    }

    fn get_ascii_char(y_prev: f64, y: f64, y_next: f64) -> char {
        if y_next > y && y_prev > y {
            '-'
        } else if y_next < y && y_prev < y {
            '-'
        } else if y_next == y && y_prev < y {
            '-'
        } else if y_next < y && y_prev == y {
            '-'
        } else if y_next > y {
            '/'
        } else if y_next < y {
            '\\'
        } else if y_next == y {
            '-'
        } else if y_prev < y {
            '/'
        } else if y_prev > y {
            '\\'
        } else if y_prev == y {
            '-'
        } else {
            '?'
        }
    }

    fn print_top_row(max_val: f64, max_width: &u16) {
        let max_str = "* Upper Value :".to_owned() + &format!("{:.2} ", max_val);
        print!("{}", max_str);
        for _i in 0..*max_width - max_str.len() as u16 {
            print!("*");
        }
    }
    //TODO: Insert Mean and Std+Dev
    fn print_bottom_row(min_val: f64, max_width: &u16) {
        let max_str = "* Lower Value :".to_owned() + &format!("{:.2} ", min_val);
        print!("{}", max_str);
        for _i in 0..*max_width - max_str.len() as u16 {
            print!("*");
        }
    }

    fn get_min_max<T>(values: &[T]) -> (f64, f64)
    where
        T: PartialOrd + Clone,
        f64: From<T>,
    {
        values
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |a, ref b| {
                (
                    a.0.min(f64::from((**b).clone())),
                    a.1.max(f64::from((**b).clone())),
                )
            })
    }

    pub fn gen_graph<T>(values: Vec<T>, height: Option<i16>, width: Option<u16>)
    where
        T: PartialOrd + Display + Debug + Clone,
        f64: From<T>,
    {
        let min_max = get_min_max(&values);
        let max_width = width.unwrap_or(180);
        let max_height = height.unwrap_or(min_max.1.min(20_f64) as i16);
        let mut adjusted_value = gen_scale_x_values(&values, max_width);
        let (min_val, max_val) = get_min_max(&values);
        adjusted_value = scale_y_value(&adjusted_value[..], 0, max_height, false);
        adjusted_value = adjusted_value.iter().map(|x| x.round()).collect();
        let field = get_ascii_field(adjusted_value.to_owned());
        print_top_row(max_val, &max_width);
        print!("\n");
        for _i in (0..field[0].len()).rev() {
            for j in 0..adjusted_value.len() {
                print!("{}", field[j][_i]);
            }
            print!("\n");
        }
        print_bottom_row(min_val, &max_width);
    }

    fn gen_mean<'a, T>(values: &'a [T]) -> f64
    where
        T: PartialOrd + Display + Debug + Clone,
        f64: From<T>,
    {
        values
            .iter()
            .fold(0_f64, |a, ref b| a + f64::from((**b).clone())) / values.len() as f64
    }

    fn gen_standard_deviation<'a, T>(values: &'a Vec<T>, mean: Option<f64>) -> f64
    where
        T: PartialOrd + Display + Debug + Clone,
        f64: From<T>,
    {
        let mean = mean.unwrap_or(gen_mean(&values));
        let a: f64 = values
            .iter()
            .map(|x| (f64::from((x).clone()) - mean).powi(2))
            .sum();
        (a / (values.len() as f64 - 1.0)).sqrt()
    }

    fn gen_scale_x_values<'a, T>(values: &'a Vec<T>, max_width: u16) -> Vec<f64>
    where
        T: PartialOrd + Display + Debug + Clone,
        f64: From<T>,
    {
        let mut scaled_value: Vec<f64> = Vec::new();
        if values.len() as u16 > max_width {
            let get_position = |i: u16| -> usize {
                let pos: u32 = ((values.len() as u32) * i as u32) as u32 / max_width as u32;
                pos as usize
            };

            for i in 0..max_width {
                scaled_value.push(gen_mean(&values[get_position(i)..get_position(i + 1)]));
            }
        } else {
            for i in values {
                scaled_value.push(f64::from((*i).clone()))
            }
        }
        scaled_value.to_owned()
    }

    #[test]
    fn test_min_max_float() {
        assert_eq!(
            get_min_max(&vec![7.0, 6.0, 5.0, 4.5, 3.5, 2.7, 1.0]),
            (1.0, 7.0)
        );
    }

    #[test]
    fn test_min_max_int() {
        assert_eq!(get_min_max(&vec![7, 6, 5, 4, 3, 2, 1]), (1.0, 7.0));
    }

    #[test]
    fn test_calc_mean() {
        let val = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(gen_mean(&val), 3.0);
    }

    //
    //    #[test]
    //    fn test_calc_mean_10() {
    //        let val = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //        assert_eq!(mean(&val[..]), 5.5);
    //    }
    //
    //    #[test]
    //    fn test_calc_mean_100() {
    //        let mut val: Vec<i32> = Vec::new();
    //        for i in 1..=100 {
    //            val.push(i);
    //        }
    //        assert_eq!(mean(&val[..]), 50.5);
    //    }
    //
    #[test]
    fn test_calc_stddev() {
        let val = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        //assert_eq!(standard_deviation(&val[..]), 1.5811388300841898);
        assert_eq!(gen_standard_deviation(&val, None), 1.5811388300841898);
    }
    //
    //    #[test]
    //    fn test_calc_stddev_10() {
    //        let val = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //        assert_eq!(standard_deviation(&val[..]), 3.0276503540974917);
    //    }
    //
    //    #[test]
    //    fn test_calc_stddev_100() {
    //        let mut val: Vec<i32> = Vec::new();
    //        for i in 1..=100 {
    //            val.push(i);
    //        }
    //        assert_eq!(standard_deviation(&val[..]), 29.011491975882016);
    //    }
    //
    //    #[test]
    //    fn test_scale_x() {
    //        let val: Vec<i32> = vec![1, 2, 3, 4, 5];
    //        assert_eq!(scale_x_values(&val[..], 12), vec![1.0, 2.0, 3.0, 4.0, 5.0])
    //    }
}
