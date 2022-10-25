#!/usr/bin/python3

from collections import defaultdict
shellcnt = defaultdict(int)

with open ('passwd', "r") as pw:
  for line = pw.readline():
    pwent = line.split(":")
    shell = pwent[6].rstrip()
    shellcnt[shell] +=1;

shells = sorted(shellcnt.keys())

for i in shells:
  print("{0:18} {1:5}".format(i, shellcnt[i]))

