#include <stdio.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <tbs/types.h>

/* the 64-bit FNV-1a constants */
#define FNV_PRIME 0x00000100000001B3
#define FNV_OFFSET 0xcbf29ce484222325

#define likely(x) \
	__builtin_expect((x),1)

#define unlikely(x) \
	__builtin_expect((x),0)

/* we could try only hasing the first few characters */
static inline long fnv(const char *s, int len)
{
	long hash = FNV_OFFSET;
	for(int i = 0; i < len; ++i)
	{
		hash *= FNV_PRIME;
		hash ^= s[i];
	}

	return hash;
}

struct shell
{
	char name[32];
	int occurrences;
	u32 hash;
};

int main()
{
	int fd = open("passwd", O_RDONLY);

	/* indexed via hash. we allocate for 256 items, because n % 2^k is
	 * equivelant to n & 2^k. even better, the modulo will come as a natural
	 * side effect of finitely-wide integers (overflow :3) */
	struct shell shells[256];

	memset(shells, 0, sizeof(shells));

	/* pointer to last encountered delimiter */
	const char *colon;

	/* allocate a couple extra bytes so we can read multiple at a time without worrying about segfaulting >:) */
	size_t fs = lseek(fd, 0, SEEK_END) + sizeof(long);

	/* entire file is mapped into memory */
	long *contents = mmap(0, fs, PROT_READ | PROT_WRITE, MAP_PRIVATE, fd, 0);

	while(((char*)contents)[0] != 0)
	{
#define elif8 \
	     X(7) \
	else X(6) \
	else X(5) \
	else X(4) \
	else X(3) \
	else X(2) \
	else X(1) \
	else X(0)

#define for8 \
	X(0) \
	X(1) \
	X(2) \
	X(3) \
	X(4) \
	X(5) \
	X(6) \
	X(7)
		/* should only access word[0] - word[7] */
		const char *word = (char*)contents;


		/* very important that we check for eol first so we don't hook onto a
		 * colon that occurs after the eol */

#define X(n) \
		if(unlikely(word[n] == '\n')) \
		{ \
			const char *ac = colon + 1; \
			int len = (word + n) - ac; \
			u8 hash = fnv(ac, len); \
			struct shell *sh = &shells[hash]; \
			if(unlikely(sh->name[0] == 0)) \
				memcpy(sh->name, ac, len); \
			++sh->occurrences; \
		}
		elif8
#undef X

#define X(n) \
		if(unlikely(word[n] == ':')) \
			colon = word + n;
		elif8
#undef X

		++contents;
	}

	/* print results by iterating `shells` and skipping entries with no occurrences */
	for(int i = 0; i < 256; ++i)
	{
		const struct shell *sh = shells + i;

		if(unlikely(sh->occurrences > 0))
		{
			char buf[128];
			char *p = buf;
			int uhh;

			for(const char *c = sh->name; *c; ++c)
				*p++ = *c;

			p[+0] = ' ';
			p[+1] = ':';
			p[+2] = ' ';
			p += 3;

			/* OPTIMISATION: custom itoa? */
			p += sprintf(p, "%d", sh->occurrences);

			*p++ = '\n';
			write(STDOUT_FILENO, buf, p - buf);
		}
	}

	/* clean up (seems to be faster than letting the OS do it lol) */
	munmap(contents, fs);
	close(fd);
}

