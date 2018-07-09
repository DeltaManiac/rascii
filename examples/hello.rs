extern crate rasciigraph;
fn main() {
    //rasciigraph::grapher::gen_graph(vec![7.0, 6.0, 5.0, 4.5, 3.5, 2.7, 1_f64], None, None);
//    rasciigraph::grapher::gen_graph(
//        vec![
//            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
//        ],
//        None,
//        None,
//    );
    /*rasciigraph::grapher::gen_graph(
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
        None,
        None,
    );
    
    rasciigraph::grapher::gen_graph(
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        ],
        None,
        None,
    );
    rasciigraph::grapher::gen_graph(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
        None,
        None,
    );
    rasciigraph::grapher::gen_graph(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        None,
        None,
    );

    rasciigraph::grapher::gen_graph(
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ],
        None,
        None,
    );

    rasciigraph::grapher::gen_graph(
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        ],
        None,
        None,
    );
    rasciigraph::grapher::gen_graph(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
        None,
        None,
    );
    rasciigraph::grapher::gen_graph(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        None,
        None,
    );*/

    let mut rand_vec = Vec::new();
    for i in 0..200 {
        rand_vec.push(f64::sin(i as f64) * 10.0);
        //rand_vec.push(i as i32);
    }
    rasciigraph::grapher::gen_graph(rand_vec, None, Some(23));
    //rasciigraph::grapher::gen_graph(vec![1231, 223, 33123, 423, 125, 6, 7], Some(20), None);
    //rasciigraph::grapher::graph(vec![7.0, 6.0, 5.0, 4.5, 3.5,2.7, 1_f64], None, None);
        /*rasciigraph::grapher::graph(vec![1, 2, 3, 4, 5, 6, 7], None, None);
rasciigraph::grapher::graph(vec![1, 2, 3, 4, 5, 6, 7], Some(20), None);
rasciigraph::grapher::graph(vec![1, 2, 3, 4, 5, 6, 7], None, Some(20));
rasciigraph::grapher::graph(vec![1, 2, 3, 4, 5, 6, 7], Some(20), Some(20));
rasciigraph::grapher::graph(vec![10, 2, 3, 4, 5, 6, 7], None, None);
rasciigraph::grapher::graph(vec![7, 6, 5, 4, 3, 2, 1], None, None);
rasciigraph::grapher::graph(
vec![7, 7, 7, 6, 5, 5, 5, 5, 5, 4, 3, 2, 2, 2, 2, 2, 2, 2, 1],
None,
None,
);
rasciigraph::grapher::graph(vec![7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7], None, None);
rasciigraph::grapher::graph(vec![20, 2, 130, 2, 600, 50, 80, 30, 90], None, None)
*/
    //rasciigraph::grapher::graph(
    //vec![7.2, 7.3, 7.4, 6.4, 5.2, 5.0, 5.1, 5.9, 5.7, 4.2, 3.3, 2.6, 72.0, 2.1, 2.3, 3.2, 6.2, 2.2, 1.1],
    //
    //None,
    //None,
    //);
}
