// extern crate statistical;
// extern crate statrs;
pub mod a {
    use std::cmp;
    use std::f64;
    // use statistical::{mean,standard_deviation};
    // use statrs::statistics::Variance;
    pub fn hello() {
        println!("HEIIII");
    }

    fn mean(values: &[i32]) -> f64 {
        let sum: i32 = values.iter().sum();
        (sum as f64 / (values.len() as f64))
    }

    fn standard_deviation(values: &[i32]) -> f64 {
        let mean = mean(&values[..]);
        let a: f64 = values.iter().map(|x| ((*x as f64) - mean).powi(2)).sum();
        (a / (values.len() as f64 - 1.0)).sqrt()
    }

    fn scale_x_values(values: &[i32], max_width: i32) -> Vec<f64> {
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
        println!("y_min {}", y_min_value);
        let y_max_value = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        println!("ymax {}", y_max_value);
        let new_min = 0;
        let mut old_range = y_max_value - y_min_value;
        if old_range == 0.0 {
            old_range = 1.0;
        };
        println!(" old range {}", old_range);
        let new_range = new_max - new_min;
        println!(" new range {}", new_range);
        for i in values {
            let a: f64 = (i - y_min_value) as f64;
            println!("a {}", a);
            let b: f64 = a * new_range as f64;
            println!("b {}", b);
            let c: f64 = b / old_range as f64;
            println!("c {}", c);
            let d: f64 = c + new_min as f64;
            println!("d {}", d);
            scaled_value.push(d);
            println!("scaled {:?}", scaled_value);
        }
        scaled_value.to_owned()
    }

    fn get_ascii_field(values: Vec<f64>) ->Vec<Vec<char>>{
        let _empty_space = ' ';
        let mut field: Vec<Vec<char>> = vec![];
//        for _i in 0..(values.len() as i32 + 1) {
//            let mut temp: Vec<char> = Vec::new();
//            for _j in 0..(values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)) as i32) + 1{
//                temp.push(_empty_space)
//            }
//            field.push(temp);
//        }
        for _i in  0..(values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)) as i32){
            let mut temp: Vec<char> = Vec::new();
            for _j in 0..(values.len() as i32 + 1){
                temp.push(_empty_space)
            }
            field.push(temp);
        }
        for x in (0..(values.len()-1)) as usize {
            let y = values[x];
            let y_prev = if x != 0 { values[x - 1] } else { y };
            let y_next = if x != values.len()-1 { values[x + 1] } else { y };
            println!("x:{} , y:{} , field_len:{}, field_inner.len:{}",x,y,field.len(),field[0].len());
            if ((y_prev - y) as i32).abs() > 1 {
                let y = if y_prev - y > 0.0 { 1 } else { -1 };
            }
            field[x as usize][y as usize] = get_ascii_char(y_prev,y,y_next)
        }
        println!("{:?}", field);
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

    pub fn rasciigraph(values: Vec<i32>, height: Option<i32>, width: Option<i32>) {
        let border_char = '*';
        let max_width = width.unwrap_or(180);
        let max_height = height.unwrap_or(cmp::min(20, *values.iter().max().unwrap()));
        println!("{} m {} ", max_height, max_width);
        let mean = mean(&values[..]);
        println!("MEAN {}", mean);
        let std_dev = standard_deviation(&values[..]);
        println!("std dev {:?}", std_dev);
        let mut adjusted_value = scale_x_values(&values[..], max_width);
        println!(" adjussted {:?}", adjusted_value);
        adjusted_value = scale_y_value(&adjusted_value[..], 0, max_height, false);
        println!(" adjussted {:?}", adjusted_value);
        adjusted_value = adjusted_value.iter().map(|x| x.round()).collect();
        println!(" adjussted {:?}", adjusted_value);
        let field = get_ascii_field(adjusted_value);
        for i in field.iter(){
            for j in i.iter().rev(){
                print!("{}",j);
            }
            print!("\n");
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
