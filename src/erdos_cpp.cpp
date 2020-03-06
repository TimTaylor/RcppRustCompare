#include <Rcpp.h>
using namespace Rcpp;

List erdos_rcpp(int n, double p)
{
    std::vector<std::vector<int>> adj(n);

    for (int i = 1; i < n; i++)
    {
        for (int j = i + 1; j < n + 1; j++)
        {
            if (R::runif(0, 1) < p)
            {
                adj[i - 1].push_back(j);
                adj[j - 1].push_back(i);
            }
        }
    }
    List network = wrap(adj);
    return network;
}

RcppExport SEXP _RcppRustCompare_erdos_rcpp(SEXP nSEXP, SEXP pSEXP)
{
    BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter<int>::type n(nSEXP);
    Rcpp::traits::input_parameter<double>::type p(pSEXP);
    rcpp_result_gen = Rcpp::wrap(erdos_rcpp(n, p));
    return rcpp_result_gen;
    END_RCPP
}