#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static __always_inline char* itoa(unsigned short value, char* result) {
	int i = 0;
	while (value) {
		result[9 - i++] = (value % 10) + '0';
		value /= 10;
	}
	result[10] = '\0';
	return result+(10-i);
}

int main() {
  struct shell {
    char name[20];
	int count; 
  };

  void *heap =
      calloc(1,128*(sizeof(struct shell)+1)); // combine allocations for both into one fat one
  struct shell *restrict buf = heap; 
  char *buffer = heap + (sizeof(struct shell) * 128);

  FILE *fp = fopen("passwd", "r");
  size_t bufSize = 128;
  size_t lineSize;

  size_t ts = 0;

  while ((lineSize = getline(&buffer, &bufSize, fp)) != (size_t)-1) {
	const char* const colon = memrchr(buffer, ':', lineSize-6) + 1;			// TODO: Replace memrchr with precalculated positions once I figure out the math
    const size_t length = buffer + lineSize - colon - 1;
	// new id generater by @crumbtoo 
	const size_t id = (buffer[lineSize - 4] ^ length + buffer[lineSize - 5]) & 0x3f; // hash(colon);
    buf[id].count++;
    if (0 == *buf[id].name) {
      memcpy(buf[id].name, colon, length);
      buf[id].name[length] = '\t';
	  buf[id].name[length+1] = '\0';
      ts++;
    }
  }

  for (int i = 0; i < 100; i++) {
    struct shell s = buf[i];
	  if (s.count > 0) {
		  fputs(s.name, stdout);
		  puts(itoa(s.count, buf));
      if (!--ts)
        break;
    }
  }

  free(heap);
  fclose(fp);
}
