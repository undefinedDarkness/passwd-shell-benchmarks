#!/usr/bin/env crystal

# Define a Hash to store the count of each login shell
shellcnt = Hash(String, Int32).new(0)

# Open the passwd file and iterate over each line
File.open("passwd") do |file|
  file.each_line do |line|
    # Split the line into fields using colon as the delimiter
    fields = line.chomp.split(':')
    # Extract the login shell from the last field
    shell = fields[-1]
    # Increment the count of this login shell in the Hash
    shellcnt[shell] += 1
  end
end

# Iterate over the Hash and print the counts for each login shell
shellcnt.each do |key, value|
  printf("%-20s%-8s%-d\n", key, ":", value)
end
