#include <stdio.h>

extern const char *get_string(char);
int main() {
  const char *string = get_string(9);
  printf("%s", string);
}
