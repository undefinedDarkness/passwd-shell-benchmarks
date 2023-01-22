#!/usr/bin/env ruby

shellcnt = {}

if File.exists?("passwd")
   f = File.foreach("passwd") do |line|
   line.chomp!
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

