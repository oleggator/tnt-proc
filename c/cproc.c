#include <tarantool/module.h>
#include <stdio.h>
#include <msgpuck.h>

int cproc(box_function_ctx_t *ctx, const char *args, const char *args_end) {
    printf("%s\n", "c procedure");

    box_tuple_format_t *fmt = box_tuple_format_default();

    const int buffer_len = 128;
    char buffer[buffer_len];

    char *end = mp_encode_str(buffer, "response", 8);
    box_tuple_t *tuple = box_tuple_new(fmt, buffer, end);
    box_return_tuple(ctx, tuple);

    return 0;
}
