#include <stdio.h>
#include <string.h>
// demo program to read a unix password file and show 
// how many instances of each login shell are found
// unlike perl/python, c doesn't have associative arrays, so....

int main()
{
   FILE *fp1;
   char line[256], shell[32];
   char shells[16][32];
   int shellcnt[16];
   int i, j, k=0, numshells=0, len, mflag;

   // initialize shell and shellcount arrays

   for( i=0; i<16; i++) {
	for (j = 0; j < 32; j++) {
	    shells[i][j] = '\0';
	}
   }

   for( i=0; i<16; i++) {
	shellcnt[i] = 0;
   }

   // read each passwd line and get the shell
   fp1= fopen ("passwd", "r");
   while (fgets(line, sizeof(line), fp1)) {
	len = strlen(line);
	for (j = len; j >= 0; j--) {
	   if (line[j] == ':') {
	     strncpy(shell, &line[++j], 32);
	     shell[strlen(shell) -1] = '\0';
	     break;
	   }
	}

	// if there are no shells, add immediately to first slot
	if (numshells == 0) {
	   strcpy(shells[0], shell);
	   shellcnt[0] = 1;
	   numshells = 1;
	} else {
	   // check to see if shell already in array 
	   mflag = 0;
	   for(k = 0; k < numshells; k++) {
		if (strcmp(shells[k], shell) == 0) {
		   mflag = 1;
		   shellcnt[k] += 1;
	      } 
	}

	// no match, add new shell entry 
	if (mflag == 0) {
	   strcpy(shells[numshells], shell);
	   shellcnt[numshells] +=1;
	   numshells++;
	}
 
	}
    }
    fclose(fp1);


    // display shell tally
    for ( i = 0; i < numshells; i++ ) {
	printf("%-18s:\t%d\n", shells[i], shellcnt[i]);
    }
}
