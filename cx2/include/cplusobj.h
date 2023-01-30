#pragma once
#include <memory>

class CPlusObj {
public:
  CPlusObj();
};

std::unique_ptr<CPlusObj> new_CplusObj();