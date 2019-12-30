#include <tarantool/module.h>
#include <stdio.h>

int cproc(box_function_ctx_t *ctx, const char *args, const char *args_end) {
    printf("%s\n", "c procedure");

    return 0;
}
