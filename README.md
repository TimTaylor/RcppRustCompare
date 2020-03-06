
<!-- README.md is generated from README.Rmd. Please edit that file -->

# RcppRustCompare

<!-- badges: start -->

[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://www.tidyverse.org/lifecycle/#experimental)
<!-- badges: end -->

Quick comparison of Rust/Rcpp integration in R vis-a-vis the generation
of Erdos-Renyi random graphs (adjacency lists).

## Rust installation

Visit <https://www.rust-lang.org/tools/install> for information on
installing Rust. Note that `cargo` is only needed at build-time.

## Package installation

`devtools::install_github("tjtnew/RcppRustCompare")`

## Example

``` r
library(RcppRustCompare)
library(ggraph)
library(tidygraph)

# small graphs
n <- 50 # size of graph
p <- 0.2 # probability of edge between two nodes

ggraph(as_tbl_graph(erdos_rust(n, p))) +
  geom_edge_link() +
  geom_node_point()
#> Using `stress` as default layout
```

<img src="man/figures/README-unnamed-chunk-2-1.png" width="100%" />

``` r

ggraph(as_tbl_graph(erdos_rcpp(n, p))) +
  geom_edge_link() +
  geom_node_point()
#> Using `stress` as default layout
```

<img src="man/figures/README-unnamed-chunk-2-2.png" width="100%" />

``` r


## larger graphs - we check average degree is roughly p * (n - 1)
n <- 10000
avk <- 5
p <- avk / (n - 1)
graph_rust <- erdos_rust(n, p)
graph_rcpp <- erdos_rcpp(n, p)

mean(sapply(graph_rust, length))
#> [1] 5.005
mean(sapply(graph_rcpp, length))
#> [1] 5.01
```

## benchmarks - dense graphs

Comparable performance between the Rcpp implementation and Rust.

``` r
library(microbenchmark)

microbenchmark(
  erdos_rust(10000, 0.1),
  erdos_rcpp(10000, 0.1),
  erdos_rust(10000, 0.2),
  erdos_rcpp(10000, 0.2),
  erdos_rust(10000, 0.4),
  erdos_rcpp(10000, 0.4),
  erdos_rust(10000, 0.8),
  erdos_rcpp(10000, 0.8),
  times = 10
)
#> Unit: milliseconds
#>                    expr    min     lq   mean median     uq    max neval
#>  erdos_rust(10000, 0.1)  476.2  481.2  487.2  485.6  489.8  513.3    10
#>  erdos_rcpp(10000, 0.1)  765.9  769.5  777.3  772.0  775.1  808.0    10
#>  erdos_rust(10000, 0.2)  574.8  576.7  585.1  582.7  591.5  602.2    10
#>  erdos_rcpp(10000, 0.2) 1017.8 1028.8 1034.3 1033.2 1042.5 1047.9    10
#>  erdos_rust(10000, 0.4)  751.1  753.9  772.4  767.9  792.3  797.0    10
#>  erdos_rcpp(10000, 0.4) 1546.3 1557.1 1579.4 1574.0 1588.3 1655.0    10
#>  erdos_rust(10000, 0.8)  839.1  900.7  917.3  930.4  943.1  983.1    10
#>  erdos_rcpp(10000, 0.8) 2106.3 2145.4 2186.9 2189.0 2243.4 2249.7    10
```

## benchmarks - sparse graph

For sparse graphs we can use the algorithm from Batagelj and Brandes,
[Efficient Generation of Large Random
Networks](https://journals.aps.org/pre/abstract/10.1103/PhysRevE.71.036113).

``` r
n <- 1000000
avk <- 5
p <- avk / (n - 1)
microbenchmark(fast_erdos_rust(n, p), times = 10)
#> Unit: milliseconds
#>                   expr   min    lq  mean median    uq   max neval
#>  fast_erdos_rust(n, p) 638.6 639.7 655.7    647 663.2 714.7    10
```

This even compares well with the implementation in igraph (although
igraph has a far more complete graph representation than this toy code):

``` r
library(igraph)
microbenchmark(sample_gnp(n, p), times = 10)
#> Unit: milliseconds
#>              expr   min    lq mean median    uq   max neval
#>  sample_gnp(n, p) 606.6 787.9  801  806.9 812.6 951.3    10
```
