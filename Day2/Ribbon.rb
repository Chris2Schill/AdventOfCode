sum = 0
File.readlines('input.txt').each do |line|
    present = line.chomp.split('x').map{|i| i.to_i}
    sum += present.inject(:*)
    present.sort[0..1].each do |s|
        sum += s*2
    end
end

p sum
