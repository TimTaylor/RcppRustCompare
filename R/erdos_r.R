#' Undirected Erdos-Renyi random graph (R implementation)
#'
#' \code{erdos_r} creates an adjacency list representation of an
#' undirected Erdos-Renyi random graph.
#'
#' This function generates an undirected random graph of size \code{n}, with
#' the probability of any two nodes being connected, \code{p}.  Self and
#' repeated edges are not permitted.
#'
#' @param n number of vertices / nodes in the network.
#' @param p probability of edge between two nodes.
#' @return adjacency list representation of the resultant network
#' @examples
#' gr <- erdos_list(1000, 10)
erdos_r <- function(n, p) {
    network <- vector("list", n)
    # initialise list for better behaviour and compatibility with c++ code
    for (i in 1:n) {
        network[[i]] <- integer()
    }

    for (i in 1:(n - 1)) {
        for (j in (i + 1):n) {
            if (stats::runif(1) < p) {
                network[[i]] <- c(network[[i]], j)
                network[[j]] <- c(network[[j]], i)
            }
        }
    }
    return(network)
}