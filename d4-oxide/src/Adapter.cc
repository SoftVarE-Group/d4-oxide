#include "Adapter.h"

#include <cstdint>
#include <string>
#include <vector>

#include "boost/multiprecision/gmp.hpp"
#include "boost/multiprecision/cpp_int.hpp"

#include "src/config/ConfigConverter.hpp"
#include "src/methods/DpllStyleMethod.hpp"
#include "src/methods/MethodManager.hpp"
#include "src/methods/ProjMCMethod.hpp"
#include "src/partitioner/PartitionerKahyparMT.hpp"

namespace mpz = boost::multiprecision;

void compile_ddnnf(rust::String input, rust::String output) {
    std::ostringstream out;
    d4::Config config = d4::Config::default_values();
    config.method = "ddnnf-compiler";
    config.input = std::string(input);
    config.dump_ddnnf = std::string(output);
    d4::PartitionerKahyparMT::initPartitioner(config);
    d4::MethodManager *methodManager = d4::MethodManager::makeMethodManager(config, out);
    methodManager->run(config);
    delete methodManager;
}

void compile_ddnnf_proj(rust::String input, rust::String output) {
    std::ostringstream out;
    d4::Config config = d4::Config::default_values();
    config.method = "proj-ddnnf-compiler";
    config.input = std::string(input);
    config.dump_ddnnf = std::string(output);
    d4::PartitionerKahyparMT::initPartitioner(config);
    d4::MethodManager *methodManager = d4::MethodManager::makeMethodManager(config, out);
    methodManager->run(config);
    delete methodManager;
}

rust::Vec<unsigned char> count(rust::String input) {
  std::ostringstream out;
  d4::Config config = d4::Config::default_values();
  config.method = "counting";
  config.input = std::string(input);

  d4::PartitionerKahyparMT::initPartitioner(config);
  d4::ProblemManager *initProblem = d4::ProblemManager::makeProblemManager(config, out);
  assert(initProblem);

  d4::LastBreathPreproc lastBreath;
  d4::ProblemManager *runProblem = d4::MethodManager::runPreproc(config, initProblem, out, lastBreath);
  delete initProblem;

  auto method = new d4::DpllStyleMethod<mpz::mpz_int, mpz::mpz_int>(config, config.method, false, runProblem, out, lastBreath);
  mpz::mpz_int result = method->run_and_return();
  delete method;

  rust::Vec<unsigned char> bytes;
  auto converted = result.convert_to<mpz::cpp_int>();
  mpz::export_bits(converted, std::back_inserter(bytes), 8);
  return bytes;
}

rust::Vec<unsigned char> count_proj(rust::String input) {
  std::ostringstream out;
  d4::Config config = d4::Config::default_values();
  config.method = "projMC";
  config.input = std::string(input);

  d4::PartitionerKahyparMT::initPartitioner(config);
  d4::ProblemManager *initProblem = d4::ProblemManager::makeProblemManager(config, out);
  assert(initProblem);

  d4::LastBreathPreproc lastBreath;
  d4::ProblemManager *runProblem = d4::MethodManager::runPreproc(config, initProblem, out, lastBreath);
  delete initProblem;

  auto method = new d4::ProjMCMethod<mpz::mpz_int>(config, false, runProblem, lastBreath);
  mpz::mpz_int result = method->run_and_return();
  delete method;

  rust::Vec<unsigned char> bytes;
  auto converted = result.convert_to<mpz::cpp_int>();
  mpz::export_bits(converted, std::back_inserter(bytes), 8);
  return bytes;
}
