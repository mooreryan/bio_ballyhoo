#!/usr/bin/env ruby

Signal.trap("PIPE", "EXIT")

def header_line? line
  line.start_with? "residue"
end

header = []
residue_idx = 0
scoring_matrix = {}
File.open(ARGV[0], "rt").each_line do |line|
  line.chomp!

  unless line.start_with? "#"
    if header_line? line
      header = line.split(" ").drop 1
    else
      residue, *scores = line.split " "

      unless header[residue_idx] == residue
        abort "ERROR -- header doesn't match residue for residue " \
              "#{residue_idx + 1}"
      end

      unless scores.length == header.length
        abort "ERROR -- Length mismatch between header and data row " \
              "#{residue_idx + 1}"
      end

      if scoring_matrix.has_key? residue
        abort "ERROR -- residue #{residue} is repeated in the data rows"
      end

      scoring_matrix[residue] = header.zip(scores).to_h

      residue_idx += 1
    end
  end
end

scoring_matrix.each do |current, others|
  others.each do |(other, score)|
    puts "(b'#{current}', b'#{other}') => Some(#{score}),"
  end
end

puts "(_, _) => None,"
