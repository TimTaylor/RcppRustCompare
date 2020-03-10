
<!-- README.md is generated from README.Rmd. Please edit that file -->

# RcppRustCompare

<!-- badges: start -->

[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://www.tidyverse.org/lifecycle/#experimental)
![R-CMD-check](https://github.com/tjtnew/RcppRustCompare/workflows/R-CMD-check/badge.svg?branch=master)
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
#> Loading required package: ggplot2
library(tidygraph)
#> 
#> Attaching package: 'tidygraph'
#> The following object is masked from 'package:stats':
#> 
#>     filter

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
#> [1] 4.957
mean(sapply(graph_rcpp, length))
#> [1] 4.9778
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
#>                    expr      min       lq     mean   median       uq       max
#>  erdos_rust(10000, 0.1) 434.0135 436.3578 438.0507 436.8575 437.3080  451.2540
#>  erdos_rcpp(10000, 0.1) 468.3882 470.0834 480.9029 471.1145 474.8565  521.2699
#>  erdos_rust(10000, 0.2) 513.4064 516.7872 530.4602 521.0933 523.1697  577.7309
#>  erdos_rcpp(10000, 0.2) 566.8809 569.0271 586.3376 572.2309 618.9357  626.5065
#>  erdos_rust(10000, 0.4) 670.1642 711.0425 713.2423 713.8142 715.2291  748.6321
#>  erdos_rcpp(10000, 0.4) 746.2215 793.3890 798.5018 796.7363 811.3734  859.9656
#>  erdos_rust(10000, 0.8) 791.7960 846.4063 873.6639 873.6082 912.3787  926.3721
#>  erdos_rcpp(10000, 0.8) 825.4098 848.9763 897.0866 878.8931 897.4752 1135.6321
#>  neval
#>     10
#>     10
#>     10
#>     10
#>     10
#>     10
#>     10
#>     10
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
#>                   expr      min       lq     mean   median       uq     max
#>  fast_erdos_rust(n, p) 631.8063 680.7119 713.5019 705.5559 741.6334 800.053
#>  neval
#>     10
```

This even compares well with the implementation in igraph (although
igraph has a far more complete graph representation than this toy code):

``` r
library(igraph)
#> 
#> Attaching package: 'igraph'
#> The following object is masked from 'package:tidygraph':
#> 
#>     groups
#> The following objects are masked from 'package:stats':
#> 
#>     decompose, spectrum
#> The following object is masked from 'package:base':
#> 
#>     union
microbenchmark(sample_gnp(n, p), times = 10)
#> Unit: milliseconds
#>              expr      min      lq     mean   median       uq      max neval
#>  sample_gnp(n, p) 879.1468 935.587 983.8152 993.2981 1039.575 1062.192    10
```
