
// extern crate statistical;
// extern crate statrs;
pub mod a {
use std::cmp;
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

    fn scale_x_values(values:&[i32], max_width:i32)->&[i32]{
        if values.len() as i32 >max_width{

        }
        values
        
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
       let adjusted_value = scale_x_values(&values[..], max_width);
       println!(" adjussted {:?}", adjusted_value )
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
