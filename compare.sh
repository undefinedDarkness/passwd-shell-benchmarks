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
  CPROG=a.out
  ${CC} -O3 getshells.c
else
  echo "C Compiler not found."
fi

# Check for rust compiler
if [ -n "$(which rustc)" ];then
  RSPROG=getshells-rs
  rustc -C opt-level=3 -o getshells-rs getshells.rs 
else
  echo "rust not found"
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

# Check for awk
if [ -n "$(which awk)" ];then
  AWK=getshells.awk
else
  echo "Awk not found."
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

if [ -n "$(which ruby)" ];then
  RBPROG=getshells.rb
else
  echo "Ruby not found."
fi

if [ -n "$(which crystal)" ];then
  CRPROG=getshells-cr
  crystal build getshells.cr
  mv getshells ${CRPROG}
else
  echo "Crystal-lang not found."
fi

for i in ${CPROG} ${RSPROG} ${GOPROG} ${NODEPROG} ${PYPROG} ${PLPROG} ${JLPROG} ${LISPPROG} ${RBPROG} ${AWK} ${CRPROG} ${PSHELL} 
do
  echo "################################################"
  echo $i
  /usr/bin/time -f "\t%E real" ./${i}
done
