$grid = Array.new(1000){Array.new(1000){0}}

def get_command line
    return 'on' if line.include?('on')
    return 'off' if line.include?('off')
    return 'toggle' if line.include?('toggle')
end

def execute command, points
    if command == 'on'
        points[0].upto(points[2]) {|outer| 
            points[1].upto(points[3]) {|inner|
                $grid[outer][inner] += 1
            }
        }
    elsif command == 'off'
        points[0].upto(points[2]) {|outer| 
            points[1].upto(points[3]) {|inner|
                $grid[outer][inner] -= 1 if $grid[outer][inner] > 0
            }
        }
    else command == 'toggle'
        points[0].upto(points[2]) {|outer| 
            points[1].upto(points[3]) {|inner|
                $grid[outer][inner] = 
                   $grid[outer][inner] += 2
            }
        }
    end
end

File.readlines("input.txt").each do |line|
    command = get_command(line)
    points = line.scan(/\d+,\d+/).join(',').split(',').map{|n| n.to_i}
    execute(command,points)
end

lights_on = 0
$grid.each {|array| array.each {|light| lights_on += light}}
puts lights_on
