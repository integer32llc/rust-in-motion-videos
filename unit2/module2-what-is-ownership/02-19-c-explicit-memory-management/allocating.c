#include <stdio.h>
#include <stdlib.h>

typedef struct {
  char *name;
  int age;
} user_t;

int main(int argc, char **argv) {
  user_t *jane = malloc(sizeof(user_t));
  jane->name = "Jane";
  jane->age = 1;
  free(jane);
}