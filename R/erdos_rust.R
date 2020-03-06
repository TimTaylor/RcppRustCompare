#' Generate an erdos-renyi random graph
#'
#' @param n number of nodes
#' @param p probability of an edge between nodes
erdos_rust <- function(n, p) {
    .Call(erdos_wrapper, n, p)
}

#' Generate an erdos-renyi random graph (fast for sparse version)
#'
#' @param n number of nodes
#' @param p probability of an edge between nodes
fast_erdos_rust <- function(n, p) {
    .Call(fast_erdos_wrapper, n, p)
}