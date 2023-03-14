#include <stdio.h>

// For mmap()
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>
// For memset()
#include <string.h>

int main() {
	int fd = open("passwd", O_RDONLY);
	struct stat s;
	fstat(fd, &s);
	size_t fs = s.st_size;
	char *contents = mmap(0, fs, PROT_READ | PROT_WRITE, MAP_PRIVATE, fd, 0);

	char* shellName[128];
	int   shellCount[128];
	memset(&shellCount, 0, 512);
	//int *shellCount = calloc(100,4);

	const char *colonPos;

	while (*contents != '\0') {
		if (*contents == ':')
			colonPos = contents+1;
		else if (*contents == '\n') {
			*contents = '\0';
			size_t length = contents - colonPos;
			int id = (*(colonPos+length-3)^length + *(colonPos+length-4));//length^(*(colonPos+1) + *(colonPos+length-3)) % 50;
			shellName[id] = colonPos;
			shellCount[id]++;
		}
		contents++;
	}

	for (int i = 0; i<100;i++) {
		if (shellCount[i] > 0) {
			printf("%s\t%d\n", shellName[i], shellCount[i]);
		}
	}

	munmap(contents, fs);
	close(fd);
}
