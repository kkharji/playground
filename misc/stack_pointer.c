#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  int x;
  int y;
} foo_t;

int main(int argc, char **argv) {
  // Stack Allocation of 'a'
  int a = 5;
  // Pointer to the stack allocated int (no free required)
  int *pa = &a;
  // Mutatate both pa and what pa points to.
  *pa = 7;
  printf("--Values\na: %d, *pa: %d, &pa: %p\n\n", a, *pa, &pa);
  printf("--Addresses:\n&a: %p, pa: %p\n\n", &a, pa);

  // Pointer to a heap allocated obj of type foo_t
  foo_t *foo_heap = malloc(sizeof(foo_t));
  // Now, foo_heap should be garbage or might be 0 depending on the compiler.
  // malloc might not be 0 "That's why you see malloc followed by a memset 0"
  memset(foo_heap, 0, sizeof(foo_t));

  printf("--foo_heap initial values:\n");
  printf("foo_heap->x: %d, foo_heap->y: %d\n\n", foo_heap->x, foo_heap->y);

  // Pointer to that pointer to that obj (@conni: because it fun :D)
  foo_t **pfoo = &foo_heap;
  printf("--pfoo && foo_heap Addresses:\n");
  printf("pfoo: %p, ", pfoo);           // Points to foo_heap address
  printf("&foo_heap: %p, ", &foo_heap); // get foo_heap address
  printf("&pfoo: %p\n\n", &pfoo);       // pfoo address (different?)

  printf("--foo_heap after mutating it with a pointer that point to it:\n");
  (*pfoo)->x = 5;
  (*pfoo)->y = 7;
  printf("foo_heap->x: %d, foo_heap->y: %d\n\n", foo_heap->x, foo_heap->y);

  printf("--foo_heap after mutating it directly:\n");
  foo_heap->x = 10;
  foo_heap->y = 10;
  printf("foo_heap->x: %d, foo_heap->y: %d\n\n", foo_heap->x, foo_heap->y);

  // Array of 3 heap allocated foo_t
  foo_t *foo_array = malloc(sizeof(foo_t) * 3);
  foo_array[0].x = 5;
  foo_array[0].y = 7;

  foo_array[1].x = 10;
  foo_array[1].y = 14;

  // Yep this is valid, because you with the + operator you are moving (n *
  // sizeof(obj)) steps forward, where the next obj lies so + 0 is the first, +
  // 1 the second, + 2 the third, ...
  (foo_array + 2)->x = 15;
  (foo_array + 2)->y = 21;

  printf("0 | %d | %d\n", foo_array[0].x, foo_array[0].y);
  printf("1 | %d | %d\n", foo_array[1].x, foo_array[1].y);
  printf("2 | %d | %d\n", foo_array[2].x, foo_array[2].y);

  free(foo_array);
  free(foo_heap);

  return 0;
}
