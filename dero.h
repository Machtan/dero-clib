#include <stdint.h>

typedef int32_t dero_Status;

const dero_Status dero_OK = 0;
const dero_Status dero_ERR_NULL = -1;
const dero_Status dero_ERR_UTF8 = -2;
const dero_Status dero_ERR_CONVERT = -3;
const dero_Status dero_ERR_PANIC = -128;

const char *dero_error_message(dero_Status);

dero_Status dero_convert(const char *text, const char **output);

void dero_free_converted(const char *);

void dero_explain_error(const char *);