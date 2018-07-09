extern crate rasciigraph;
fn main() {
    rasciigraph::grapher::graph(vec![7.0, 6.0, 5.0, 4.5, 3.5, 2.7, 1_f64], None, None);
    rasciigraph::grapher::graph(
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ],
        None,
        None,
    );

    //Sine Wave ?
    let mut rand_vec = Vec::new();
    for i in 0..360 {
        rand_vec.push(f64::sin(i as f64) * 10.0);
    }
    rasciigraph::grapher::graph(rand_vec, None, None);

    rasciigraph::grapher::graph(vec![1231, 223, 33123, 423, 125, 6, 7], Some(20), Some(24));
    rasciigraph::grapher::graph(vec![7.0, 6.0, 5.0, 4.5, 3.5, 2.7, 1_f64], None, None);

    rasciigraph::grapher::graph(
        vec![
            7.2, 7.3, 7.4, 6.4, 5.2, 5.0, 5.1, 5.9, 5.7, 4.2, 3.3, 2.6, 72.0, 2.1, 2.3, 3.2, 6.2,
            2.2, 1.1,
        ],
        None,
        None,
    );
}
