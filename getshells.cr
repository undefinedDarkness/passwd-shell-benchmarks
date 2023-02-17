#!/usr/bin/env crystal

shellcnt = {} of String => Int32

if File.exists?("passwd")
   File.each_line("passwd") do |line|
      gecos = line.chomp.split(':')
      shell = gecos.last
      shellcnt[shell] ||= 0
      shellcnt[shell] += 1
   end

   shellcnt.each do |key, value|
   printf("%-20s%-8s%-d\n", key, ":", value)
   end
end
