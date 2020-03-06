context("erdos graph generation")

# test --------------------------------------------------------------------
test_that("Returned rcpp graph is correct size", {
  n <- 20
  p <- 0.4
  graph <- compareloops::erdos_rcpp(n, p)
  expect_equal(length(graph), n)
})

# test --------------------------------------------------------------------
test_that("Returned rust graph is correct size", {
  n <- 20
  p <- 0.4
  graph <- compareloops::erdos_rust(n, p)
  expect_equal(length(graph), n)
})

# test --------------------------------------------------------------------
test_that("rcpp implementation gives same answer as r implementation", {
  n <- 20
  p <- 0.4

  # generate graphs
  set.seed(1)
  graph_rcpp <- compareloops::erdos_rcpp(n, p)
  set.seed(1)
  graph_r <- compareloops::erdos_r(n, p)

  # expect
  expect_equal(graph_rcpp, graph_r)
})