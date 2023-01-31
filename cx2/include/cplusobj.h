#pragma once
#include <memory>

struct Buf;

class CPlusObj {
public:
  CPlusObj();
  ~CPlusObj() { cout << "CplusObj destroyed" << endl; }
  uint64_t put(Buf &b) const;
};

void new_CplusObj();