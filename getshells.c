#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
  /* int fd = open("passwd", O_RDONLY);
  struct stat s;
  fstat(fd, &s);
  size_t fs = s.st_size;
  char *contents = mmap(0, fs, PROT_READ | PROT_WRITE, MAP_PRIVATE, fd, 0);
  */

  struct shell {
    char name[32];
    int count;
  };

  void *heap =
      calloc(1, 512 + sizeof(struct shell) *
                          200); // combine allocations for both into one fat one
  struct shell *buf = heap; // calloc(200, sizeof(buf)); // lil extra space cuz
                            // im allocating anyway avoids memory errors
  char *buffer = heap + (sizeof(struct shell) * 200);

  FILE *fp = fopen("passwd", "r");
  size_t bufSize = 512;
  size_t lineSize;

  size_t ts = 0;

  while ((lineSize = getline(&buffer, &bufSize, fp)) != -1) {
    char *colon = buffer + lineSize - 6; // smallest is 7
    while (*colon != ':')
      colon--;
    colon++;
    size_t length = buffer + lineSize - colon - 1;
    size_t id = (buffer[lineSize - 4] ^ length + buffer[lineSize - 5]) % 50; // hash(colon);
    buf[id].count++;
    if (0 == *buf[id].name) {
      memcpy(buf[id].name, colon, length);
      buf[id].name[length] = '\0';
      ts++;
    }
  }

  for (int i = 0; i < 100; i++) {
    if (buf[i].count > 0) {
      printf("%s\t%d\n", buf[i].name, buf[i].count);
      if (!--ts)
        break;
    }
  }

  free(heap);
  fclose(fp);
  // free(buf);
  // munmap(contents, fs);
  // close(fd);
}
