#include "cx2/include/cplusobj.h"

CPlusObj::CPlusObj() {}

std::unique_ptr<CPlusObj> new_CplusObj() {
  return std::unique_ptr<CPlusObj>(new CPlusObj());
}