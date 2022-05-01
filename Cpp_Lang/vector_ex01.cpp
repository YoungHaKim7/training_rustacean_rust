#include <iostream>
#include <vector>

using namespace std;

int main(int argc, char *argv[]) {
  vector<string> v;
  v.push_back("hello");

  auto &hello = v[0];
  cout << hello.c_str() << endl;

  v.push_back("C++");
  cout << hello.c_str() << endl;

  return 0;
}
