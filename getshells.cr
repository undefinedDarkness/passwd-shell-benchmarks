#!/usr/bin/env crystal

# Create an empty hash to store shell and its count
shellcnt = Hash(String, Int32).new(0)

# Open passwd file and read line by line
File.open("passwd") do |file|
  file.each_line do |line|
    # Extract the shell from the last field of the line
    shell = line.split(':').last.chomp

    # Increment the count for the given shell in the hash
    shellcnt[shell] += 1
  end
end

# Print the shell count in formatted output
shellcnt.each do |shell, count|
  printf("%-20s%-8s%-d\n", shell, ":", count)
end
