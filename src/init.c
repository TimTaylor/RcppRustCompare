#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>

/* .Call calls */
extern SEXP _RcppRustCompare_erdos_rcpp(SEXP, SEXP);
extern SEXP erdos_wrapper(SEXP, SEXP);
extern SEXP fast_erdos_wrapper(SEXP, SEXP);

static const R_CallMethodDef CallEntries[] = {
    {"_RcppRustCompare_erdos_rcpp", (DL_FUNC)&_RcppRustCompare_erdos_rcpp, 2},
    {"erdos_wrapper", (DL_FUNC)&erdos_wrapper, 2},
    {"fast_erdos_wrapper", (DL_FUNC)&fast_erdos_wrapper, 2},
    {NULL, NULL, 0}};

void R_init_RcppRustCompare(DllInfo *dll)
{
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
