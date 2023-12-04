def double
end
count = 0
File.readlines('input.txt').each do |line|
       # Contains sandwhiched chars 'aba' 'cdc' 'zkz'
    if line.match(/([a-z]).\1/) and double(line)
        count += 1
    end
end
    
puts count
