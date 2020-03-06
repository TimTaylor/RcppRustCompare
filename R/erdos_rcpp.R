#' Undirected Erdos-Renyi random graph (Rcpp implementation)
#'
#' \code{erdos_rcpp} creates an adjacency list representation of an
#' undirected Erdos-Renyi random graph.
#'
#' This function generates an undirected random graph of size \code{n}, with
#' the probability of any two nodes being connected, \code{p}.  Self and
#' repeated edges are not permitted.
#'
#' @param n number of vertices / nodes in the network.
#' @param p probability of edge between two nodes.
#' @return adjacency list representation of the resultant network
erdos_rcpp <- function(n, p) {
    .Call("_RcppRustCompare_erdos_rcpp", PACKAGE = "RcppRustCompare", n, p)
}