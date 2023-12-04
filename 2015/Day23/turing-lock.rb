class Compiler
    attr_accessor :code, :sp, :a, :b

    def initialize code
        @code = code
        @sp = 0
        @a = 1
        @b = 0
    end

    def hlf r
        if r == 'a'
            @a = @a/2
        elsif r == 'b'
            @b = @b/2
        end
        @sp += 1
    end

    def tpl r
        if r == 'a'
            @a = @a*3
        elsif r == 'b'
            @b = @b*3
        end
        @sp += 1
    end

    def inc r
        if r == 'a'
            @a += 1
        elsif r == 'b'
            @b += 1
        end
        @sp += 1
    end

    def jmp op, offset
        @sp += offset if op == '+'
        @sp -= offset if op == '-'
    end

    def jie r, op, offset
        jumped = false
        if r == 'a' and @a.even?
            jmp(op,offset) 
            jumped = true
        elsif r == 'b' and @b.even?
            jmp(op,offset)
            jumped = true
        end
        @sp += 1 unless jumped
    end

    def jio r, op, offset
        if r == 'a'
            if @a == 1 then jmp(op,offset) else @sp += 1 end
        elsif r == 'b'
            if @b == 1 then jmp(op,offset) else @sp += 1 end
        end
    end

end

code = Array.new
File.readlines('input.txt').each {|line| code << line.chomp}
compiler = Compiler.new(code)

while 0 <= compiler.sp and compiler.sp < compiler.code.size 
    line = compiler.code[compiler.sp]
    args = line.scan(/[a-z]+|[+-]+|[0-9]+/)

    if args[0] == 'hlf'
        compiler.hlf(args[1])
    elsif args[0] == 'tpl'
        compiler.tpl(args[1])
    elsif args[0] == 'inc'
        compiler.inc(args[1])
    elsif args[0] == 'jmp'
        compiler.jmp(args[1], args[2].to_i)
    elsif args[0] == 'jie'
        compiler.jie(args[1], args[2], args[3].to_i)
    elsif args[0] == 'jio'
        compiler.jio(args[1], args[2], args[3].to_i)
    end
end

puts "Register b: #{compiler.b}"
