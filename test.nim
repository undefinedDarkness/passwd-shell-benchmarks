import strutils, tables

let passwdFile = "passwd"
var shellCounts = initTable[string, int]()

for line in passwdFile.lines:
  let fields = line.split(":")
  let shell = fields[$-1]
  shellCounts[shell] += 1

for shell, count in shellCounts.pairs:
  let countStr = $count
  let shellWidth = 25 - len(countStr)
  echo(shell & " " * shellWidth & " : " & countStr)
