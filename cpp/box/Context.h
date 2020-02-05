#ifndef PROCEDURES_CONTEXT_H
#define PROCEDURES_CONTEXT_H

#include <tarantool/module.h>
#include <msgpack.hpp>
#include <exception>

#define register_procedure(name) extern "C" {\
    int name(box_function_ctx_t *ctx, const char *args, const char *args_end) {\
        const auto context = Context(ctx, args, args_end);\
        return name(context);\
    }\
}

class Context {
private:
    box_function_ctx_t *ctx;
    const char *args, *args_end;

    size_t get_args_buf_len() const;

public:
    Context(box_function_ctx_t *ctx, const char *args, const char *args_end) :
        ctx(ctx), args(args), args_end(args_end) {}

    template<typename T>
    void decode_body(T &destination) const {
        const auto object_handle = msgpack::unpack(this->args, this->get_args_buf_len());
        const auto object = object_handle.get();
        object.convert(destination);
    }

    template<typename T>
    void return_tuple(T &tuple_body) const {
        msgpack::sbuffer sbuf;
        msgpack::pack(sbuf, tuple_body);

        const auto box_tuple = box_tuple_new(box_tuple_format_default(),
                                             sbuf.data(), sbuf.data() + sbuf.size());

        int err = box_return_tuple(this->ctx, box_tuple);
        if (err != 0) {
            throw std::bad_alloc();
        }
    }

    class Error : public std::exception {
        const char *what() const noexcept override {
            return "Tarantool Box Error";
        }
    };
};


#endif //PROCEDURES_CONTEXT_H
