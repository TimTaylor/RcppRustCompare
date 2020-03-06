#include <Rinternals.h>
#include <R.h>

#include "rustgraphs/api.h"

SEXP copy_int_vec(int *x, ptrdiff_t size)
{
    SEXP ans = allocVector(INTSXP, size);
    PROTECT(ans);
    int *pans = INTEGER(ans);
    for (ptrdiff_t i = 0; i < size; i++)
    {
        pans[i] = x[i];
    }
    UNPROTECT(1);
    return ans;
}

SEXP erdos_wrapper(SEXP nn, SEXP pp)
{
    int N = asInteger(nn);
    double p = asReal(pp);
    struct graph_buffer graph = erdos(N, p);
    SEXP ans = allocVector(VECSXP, N);
    PROTECT(ans);

    int *graph_values = graph.array;
    size_t *graph_lengths = graph.node_degrees;

    int cnt = 0;
    for (int i = 0; i < N; i++)
    {
        SET_VECTOR_ELT(ans, i, copy_int_vec(&graph_values[cnt], graph_lengths[i]));
        cnt += graph_lengths[i];
    }

    rust_free(graph);
    UNPROTECT(1);
    return ans;
}

SEXP fast_erdos_wrapper(SEXP nn, SEXP pp)
{
    int N = asInteger(nn);
    double p = asReal(pp);
    struct graph_buffer graph = fast_erdos(N, p);
    SEXP ans = allocVector(VECSXP, N);
    PROTECT(ans);

    int *graph_values = graph.array;
    size_t *graph_lengths = graph.node_degrees;

    int cnt = 0;
    for (int i = 0; i < N; i++)
    {
        SET_VECTOR_ELT(ans, i, copy_int_vec(&graph_values[cnt], graph_lengths[i]));
        cnt += graph_lengths[i];
    }

    rust_free(graph);
    UNPROTECT(1);
    return ans;
}