def surfaceArea p
    a1, a2, a3 = p[0]*p[1], p[1]*p[2], p[0]*p[2]
    a1*2 + a2*2 + a3*2 + [a1,a2,a3].min
end

sum = 0
File.readlines('input.txt').each do |line|
    present = line.chomp.split('x').map{|x| x.to_i}
    sum += surfaceArea(present) 
end

print sum
