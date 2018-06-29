extern crate rasciigraph;
fn main()
{
   rasciigraph::grapher::graph(vec![1, 2, 3, 4, 5, 6, 7,], None, None);
   rasciigraph::grapher::graph(vec![1, 2, 3, 4, 5, 6, 7,], Some(20), None);
   rasciigraph::grapher::graph(vec![1, 2, 3, 4, 5, 6, 7,], None, Some(20));
   rasciigraph::grapher::graph(vec![1, 2, 3, 4, 5, 6, 7,], Some(20), Some(20));
   rasciigraph::grapher::graph(vec![10, 2, 3, 4, 5, 6, 7,], None, None);
   rasciigraph::grapher::graph(vec![7, 6, 5, 4, 3, 2, 1,], None, None);
   rasciigraph::grapher::graph(vec![7, 7, 7, 6, 5, 5, 5, 5, 5, 4, 3, 2, 2, 2, 2, 2, 2, 2, 1,], None, None);
   rasciigraph::grapher::graph(vec![7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7], None, None);
   rasciigraph::grapher::graph(vec![20, 2, 130, 2, 600, 50, 80, 30, 90,], None, None)
}
