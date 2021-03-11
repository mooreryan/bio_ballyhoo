# Dereplicate Sequences

Give me some sequences in a fasta file, and I will remove any exact duplicates.

I am the simplest possible program you could imagine for doing this task.  I take the sequence part of each record, insert it as the key in a hash table, and let it work it's magic.  There are probably a million better ways to do this, but this way gets the job done. 

For duplicated sequences, the ones further down in the file will be kept.
