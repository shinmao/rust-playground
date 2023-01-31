#include "cx2/include/cplusobj.h"

CPlusObj::CPlusObj() {}

void new_CplusObj() {
  std::cout << "First call from main Rust to C++ new_CplusObj" << endl;

  // if dev uses smart pointer here, then they will not manually free it
  auto obj = CplusObj();

  try {
    std::cout << "Pass unique pointer to Rust side" << endl;
    next_chunk();
  } catch(...) {
    std::cout << "rust panic caught in C++" << endl;
  }

  std::cout << "catch block ended" << endl;
}