$commands = Array.new
$known_wires = Hash.new
$wires_exp = Regexp.new('([a-z]+)')
$number_exp = Regexp.new('([0-9]+)')
$op_exp = Regexp.new('([A-Z]+)')
$command_index = 0

def get_commands
    File.readlines('input.txt').each do |line|
        $commands << line.chomp 
    end
end


def get_all_inputs commands
    $commands.each do |command|
        command.split(' ').each { |w| $known_wires[w] = nil if w.match($wires_exp) } 
    end
end


def try_to_execute command
    if inputs_not_null command
        execute(command)
    else
        $command_index += 1
    end

    if $command_index >= $commands.size
        $command_index = 0
    end
end

def inputs_not_null command
    commands_wires = Array.new
    command.split(' ').each {|w| commands_wires << w if w.match($wires_exp)} 
    commands_wires.delete(commands_wires.last)
    commands_wires.each {|wire| return false if $known_wires[wire] == nil}
    return true
end


def execute command
    op = command.match($op_exp)
    destination = command.split(' ').last
    puts command
    puts "Executing op:#{op} #{$known_wires[command.split(' ').last]}" 
    if op == nil
        input = command.split(' ').first
        $known_wires[destination] = input.to_i if input.to_i.to_s == input
        $known_wires[destination] = $known_wires[command.split(' ').first] if input.to_i.to_s != input
    elsif op.to_s == 'RSHIFT'
        $known_wires[destination] = 
            $known_wires[command.split(' ').first] >> command.match($number_exp)[0].to_i  
    elsif op.to_s == 'LSHIFT'
        $known_wires[destination] = 
            $known_wires[command.split(' ').first] << command.match($number_exp)[0].to_i 
    elsif op.to_s == 'AND'
        matched = command.scan($wires_exp)
        firstInput = matched[0][0]
        secondInput = matched[1][0]
        numMatched = command.match($number_exp)
        secondInput = numMatched[0].to_i if secondInput == command.split(' ').last
        puts secondInput.is_a? Numeric
        if secondInput.is_a? Numeric
            puts 'Numeric'
            $known_wires[destination] = $known_wires[firstInput] & secondInput
        elsif secondInput.is_a? String
            puts "String"
            puts "#{firstInput} = #{$known_wires[firstInput]}, #{secondInput} = #{$known_wires[secondInput]}"
            $known_wires[destination] = $known_wires[firstInput] & $known_wires[secondInput]
        end
    elsif op.to_s == 'OR'
        matched = command.scan($wires_exp)
        firstInput = matched[0][0]
        secondInput = matched[1][0]
        puts "#{firstInput} = #{$known_wires[firstInput]}, #{secondInput} = #{$known_wires[secondInput]}"
        $known_wires[destination] = $known_wires[firstInput] | $known_wires[secondInput]
    elsif op.to_s == 'NOT'
        matched = command.scan($wires_exp)
        firstInput = matched[0][0]
        $known_wires[destination] = ~$known_wires[firstInput]
    end
    print "#{destination} = #{$known_wires[destination]}"
    $command_index = 0
    $commands.delete(command)
    puts
    puts
end

get_commands
get_all_inputs($commands)
while $commands.size > 0
    try_to_execute $commands[$command_index]
end

a = $known_wires['a']

get_commands
get_all_inputs($commands)

$known_wires['b'] = a


while $commands.size > 0
    try_to_execute $commands[$command_index]
end

puts $known_wires['a']
