/// bridge.hpp

#ifndef __BRIDGE_H__
#define __BRIDGE_H__

/**
 * Bridge for cpp
 */
class Bridge {
protected:
  char *str;
public:
  /// construct
  Bridge();
  /// construct with ptr
  Bridge(char *s);
  /// destruct
  virtual ~Bridge();
  /// assign ptr
  void pset(char *p);
  /// display
  void put();
};

extern "C" {
/// legacy C interface
void bput();
}

extern "C" {
/// adhoc C macro and static __inline functions

/// dReal as f64 (defined in ode.hpp)
typedef double dReal;

/// as local function to be independent of ode.hpp
dReal _dDot(const dReal *a, const dReal *b, int n);
}

#endif // __BRIDGE_H__
