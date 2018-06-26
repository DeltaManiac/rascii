
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

    fn mean(values:&[i32])->f64{
        let sum:i32 = values.iter().sum();
        (sum as f64/(values.len() as f64))
    }

    fn standard_deviation(values:&[i32])->f64{
        let mean = mean(&values[..]);
        let a:f64  = values.iter().map(|x| ((*x as f64) - mean).powi(2)).sum();
        let b = (a / (values.len() as f64 - 1.0) ).sqrt();
        b
        }

    fn scale_x_values<'a,'b>( values:&'a[i32], max_width:i32)->&'a[f64]{
       let mut scaled_value:Vec<f64> = Vec::new();
        if values.len() as i32 >max_width{

        }
        for i in values {
            scaled_value.push(*i as f64)
        }
       &scaled_value[..]
    }

    fn scale_y_value(values:&[f64],new_min:i32,new_max:i16,scale_from_old_zero:bool)->&[f64]{
       let mut scaled_value:Vec<f64> = Vec::new();
        let mut y_min_value = values.iter().fold(f64::INFINITY,|a,&b| a.min(b));
        if scale_from_old_zero {
            y_min_value = 0.0;
        }
        let mut y_max_value =   values.iter().fold(f64::INFINITY,|a,&b| a.min(b)); 
        let new_min = 0;
        let mut old_range = y_max_value - y_min_value;
        if old_range == 0.0{
            old_range = 1.0;
        };
        let new_range = new_max - new_min;
        for i  in values {
            let a:f64 = (i-y_min_value) as f64;
            let b:f64= a * new_range as f64;
            let c:f64 = b /old_range as f64;
            let d:f64 = c + new_min as f64;
            scaled_value.push(d);
        }
       &scaled_value[..]
    }
    pub fn rasciigraph(values:Vec<i32>,height:Option<i16>,width:Option<i32>){
        let border_char = '*';
        let max_height = height.unwrap_or(180 );
        let max_width =  width.unwrap_or(cmp::min(20,values.len() as i32));
        println!("{} m {} ",max_height, max_width );
        let mean = mean(&values[..]);
        println!("MEAN {}", mean );
       let std_dev = standard_deviation(&values[..]);
        println!("std dev {:?}", std_dev );
       let mut adjusted_value:&[f64] = scale_x_values(&values[..], max_width);
       println!(" adjussted {:?}", adjusted_value );
       adjusted_value = scale_y_value( &adjusted_value[..],0,max_height,false);
       println!(" adjussted {:?}", adjusted_value );
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
