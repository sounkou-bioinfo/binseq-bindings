#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stdlib.h>

/* Function declarations */
SEXP C_read_binseq(SEXP file);

static const R_CallMethodDef CallEntries[] = {
    {"C_read_binseq", (DL_FUNC) &C_read_binseq, 1},
    {NULL, NULL, 0}
};

void R_init_Rbinseq(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}