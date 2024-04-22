/// bridge.cpp

#include <iomanip>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <map>

#include <cstring>
#include <cmath>

using namespace std;

#include <bridge.hpp>

/// construct
Bridge::Bridge() : str("_bridge")
{
}

/// construct with ptr
Bridge::Bridge(char *s) : str(s)
{
}

/// destruct
Bridge::~Bridge()
{
}

/// assign ptr
void Bridge::pset(char *p)
{
  str = p;
}

/// display
void Bridge::put()
{
  cout << string(str) << endl;
}

/// legacy C interface
void bput()
{
  Bridge b = Bridge("bridge");
  b.put();
}

/// adhoc C macro and static __inline functions

/// as local function to be independent of ode.hpp
dReal _dDot(const dReal *a, const dReal *b, int n)
{
  dReal s = 0.0;
  for(int i = 0; i < n; ++i){ s += a[i] * b[i]; }
  return s;
}
