#!/usr/bin/env ruby

output = `cargo atcoder test a`
output = output.split('your output:')[1]
lines = output.split("\n");
lines = lines.select { |l| l =~ /^\s+\d+\s*\|\s+/ }
lines = lines.map { |l| l.gsub(/^\s+\d+\s*\|\s+/, '') }
puts lines.join("\n")