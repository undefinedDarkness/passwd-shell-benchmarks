#!/bin/sh

# Check for C Compiler
if [ -n "${CC}" ];then
  CC=${CC}
elif [ -n "$(which clang)" ];then
  CC=clang
elif [ -n "$(which gcc)" ];then
  CC=gcc
elif [ -n "$(which cc)" ];then
  CC=cc
fi
if [ -n "${CC}" ];then
  CPROG=getshells
  ${CC} -O2 -o ${CPROG} getshells.c
else
  echo "C Compiler not found."
fi

# Check for golang compiler
if [ -n "(which go)" ];then
  GOPROG=getshells-go
  go build main.go
  mv main ${GOPROG}
else
  echo "Golang compiler not found."
fi

# Check for Powershell
if [ -n "$(which pwsh)" ];then
  PSHELL=getshells.ps1
else
  echo "Powershell not found."
fi

if [ -n "$(which python3)" ];then
  PYPROG=getshells.py
else
  echo "Python3 not found."
fi

if [ -n "$(which perl)" ];then
  PLPROG=getshells.pl
else
  echo "Perl not found."
fi

if [ -n "$(which sbcl)" ];then
  LISPPROG=getshells.lisp
else
  echo "SBCL (Lisp) not found."
fi

if [ -n "$(which node)" ];then
  NODEPROG=getshells.js
else
  echo "NodeJS not found."
fi

if [ -n "$(which julia)" ];then
  JLPROG=getshells.jl
else
  echo "Julia not found."
fi

for i in ${CPROG} ${GOPROG} ${PSHELL} ${PYPROG} ${PLPROG} ${LISPPROG} ${NODEPROG} ${JLPROG} getshells.awk
do
  echo "################################################"
  echo $i
  /usr/bin/time -f "\t%E real" ./${i}
done

