#include <iostream>
#include <vector>

int main(int argc, char *argv[]) {
  std::vector<int> v1 = {1, 2, 3};
  auto v2 = v1;
  auto v3 = v1;

  std::cout << v1.size() << " " << v2.size() << " " << v3.size() << std::endl;
  return 0;
}
