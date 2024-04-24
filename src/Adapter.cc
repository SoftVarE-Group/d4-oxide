#include "Adapter.h"

#include <cstdint>
#include <string>

#include "src/config/ConfigConverter.hpp"
#include "src/methods/MethodManager.hpp"

void compile_ddnnf(rust::String input, rust::String output) {
    std::ostringstream out;

    d4::Config config = d4::Config::default_values();
    config.method = "ddnnf-compiler";
    config.input = std::string(input);
    config.dump_ddnnf = std::string(output);

    d4::MethodManager *methodManager = d4::MethodManager::makeMethodManager(config, out);
    methodManager->run(config);
    delete methodManager;
}
