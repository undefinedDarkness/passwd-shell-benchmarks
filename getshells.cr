#!/usr/bin/env crystal

shellcnt = {} of String => Int32

if File.exists?("passwd")
   f = File.each_line("passwd") do |line|
   line.chomp
   gecos = line.split(':')
   shell = gecos.last
    if shellcnt.has_key? shell
       shellcnt[shell] = shellcnt[shell] + 1
    else
       shellcnt[shell] = 1
    end
  end
  
  shellcnt.each do |key, value|
    printf("%-20s%-8s%-d\n", key, ":", value)
  end

end
