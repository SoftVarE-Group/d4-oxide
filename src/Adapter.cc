#include "Adapter.h"

#include "DdnnfCompiler.hpp"

void compile_ddnnf(rust::String input, rust::String output) {
  d4::compile_ddnnf(std::string(input), std::string(output));
}
