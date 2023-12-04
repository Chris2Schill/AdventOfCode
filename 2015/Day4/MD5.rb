require 'digest'

val = (val == nil ? 0 : val + 1) until Digest::MD5.hexdigest('iwrupvqb'+val.to_s)[0..4] == "00000"

p val
