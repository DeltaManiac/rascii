pub mod grapher {
        use std::cmp;
        use std::f64;
        use std::convert::From;
        use std::fmt::Display;
        use std::fmt::Debug;
    
        fn mean(values: &[f64]) -> f64 {
            let sum: f64 = values.iter().sum();
            (sum as f64 / (values.len() as f64))
             }
             
             fn standard_deviation(values: &[f64]) -> f64 {
             let mean = mean(&values[..]);
             let a: f64 = values.iter().map(|x| ((*x as f64) - mean).powi(2)).sum();
             (a / (values.len() as f64 - 1.0)).sqrt()
              }
              
              fn scale_x_values(values: &[f64], max_width: i32) -> Vec<f64> {
              let mut scaled_value: Vec<f64> = Vec::new();
              if values.len() as i32 > max_width {}
              for i in values {
              scaled_value.push(*i as f64)
              }
              scaled_value.to_owned()
              //    &scaled_value[..].to_owned()
              }
              
              fn scale_y_value(
        values: &[f64],
        new_min: i32,
        new_max: i32,
        scale_from_old_zero: bool,
        ) -> Vec<f64> {
             let mut scaled_value: Vec<f64> = Vec::new();
             let mut y_min_value = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
             if scale_from_old_zero {
             y_min_value = 0.0;
             }
             // // // println!("y_min {}", y_min_value);
             let y_max_value = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
             // // println!("ymax {}", y_max_value);
             let new_min = 0;
             let mut old_range = y_max_value - y_min_value;
             if old_range == 0.0 {
             old_range = 1.0;
             };
             // // println!(" old range {}", old_range);
             let new_range = new_max - new_min;
             // // println!(" new range {}", new_range);
             for i in values {
             let a: f64 = (i - y_min_value) as f64;
             // // println!("a {}", a);
             let b: f64 = a * new_range as f64;
             // // println!("b {}", b);
             let c: f64 = b / old_range as f64;
             // // println!("c {}", c);
             let d: f64 = c + new_min as f64;
             // // println!("d {}", d);
             scaled_value.push(d);
             // // println!("scaled {:?}", scaled_value);
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
             // println!("len ; {:?}", values);
             for x in 0..(values.len()) as usize {
             let y = values[x];
             let y_prev = if x != 0 { values[x - 1] } else { y };
             let y_next = if x != values.len() - 1 {
             values[x + 1]
             } else {
             y
             };
             // // println!("x:{} , y:{} , field_len:{}, field_inner.len:{}",x,y,field.len(),field[0].len());
             if ((y_prev - y) as i32).abs() > 1 {
             let step = if y_prev - y > 0.0 { 1 } else { -1 };
             // println!("step {}, y_prev {}, y {}", step, y_prev, y);
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
             for i in field.iter() {
             // println!("{:?}", i);
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
             
             fn print_top_row(max_val: f64, max_width: &i32) {
             //let mean_str = "Mean:".to_owned() + &format!("{:.5}",mean);
             let max_str = "* Upper Value :".to_owned() + &format!("{:.2} ", max_val);
             let string_offset = max_width - max_str.len() as i32;
             print!("{}", max_str);
             for i in 0..*max_width - max_str.len() as i32 {
             print!("*");
             }
             }
             
             fn print_bottom_row(min_val: f64, max_width: &i32) {
             //let mean_str = "Mean:".to_owned() + &format!("{:.5}",mean);
             let max_str = "* Lower Value :".to_owned() + &format!("{:.2} ", min_val);
             let string_offset = max_width - max_str.len() as i32;
             print!("{}", max_str);
             for i in 0..*max_width - max_str.len() as i32 {
             print!("*");
             }
             }
             
             
             
             
             fn get_min_max<T>(values:Vec<T>)->(f64,f64)
             where 
             T: PartialOrd + Clone,
              f64: From<T>
             {
             values
             .iter()
             .fold((f64::INFINITY, f64::NEG_INFINITY), |a, ref b| (
            a.0.min((f64::from((**b).clone()))),
            a.1.max((f64::from((**b).clone())))
            ))
             }
             
             
             pub fn gen_graph<T>(values: Vec<T>, height: Option<i16>, width: Option<i16>)
             where
             T: PartialOrd + Display + Debug + Clone,
             f64: From<T>,
             {
            println!("{:?}",get_min_max(values));
            
    }
    
    
    pub fn graph(values: Vec<f64>, height: Option<i32>, width: Option<i32>) {
            let border_char = '*';
            let max_width = width.unwrap_or(180);
            //let max_height = height.unwrap_or(cmp::min(20, *values.iter().max().unwrap()));
            let max_height = height.unwrap_or(values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)).min(20 as f64) as i32) ;
            // println!("{} m {} ", max_height, max_width);
            let mean = mean(&values[..]);
            // println!("MEAN {}", mean);
            let std_dev = standard_deviation(&values[..]);
            // println!("std dev {:?}", std_dev);
            // // println!(" values BEFOREE {:?}", values);
            let mut adjusted_value = scale_x_values(&values[..], max_width);
            let max_val = adjusted_value
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let min_val = adjusted_value.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            // println!(" adjussted {:?}", adjusted_value);
            adjusted_value = scale_y_value(&adjusted_value[..], 0, max_height, false);
            // // println!(" values aFTER {:?}", values);
            // println!(" adjussted {:?}", adjusted_value);
            adjusted_value = adjusted_value.iter().map(|x| x.round()).collect();
            // println!(" adjussted {:?}", adjusted_value.to_owned());
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
            print!("\n");
    }
    
    //    #[test]
    //    fn test_calc_mean() {
    //        let val = vec![1, 2, 3, 4, 5];
    //        assert_eq!(mean(&val[..]), 3.0);
    //    }
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
    //    #[test]
    //    fn test_calc_stddev() {
    //        let val = vec![1, 2, 3, 4, 5];
    //        assert_eq!(standard_deviation(&val[..]), 1.5811388300841898);
    //    }
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
