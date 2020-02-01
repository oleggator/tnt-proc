#include "Context.h"

size_t Context::get_args_buf_len() const {
    return this->args_end - this->args;
}
