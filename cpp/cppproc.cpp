#include "cppproc.h"

#include <string>
#include <iostream>
#include <tuple>

int cppproc(const Context &ctx) {
    using std::string, std::cout, std::endl;
    cout << "c++ procedure" << endl;

    std::tuple<string, string, string> body;
    ctx.decode_body(body);

    const auto&[uuid, some_str, another_str] = body;
    cout << quoted(uuid) << endl
         << quoted(some_str) << endl
         << quoted(another_str) << endl;

    const auto response = std::make_tuple("response");
    try {
        ctx.return_tuple(response);
    } catch (const std::exception &e) {
        cout << "tuple return error: " << e.what() << endl;
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
