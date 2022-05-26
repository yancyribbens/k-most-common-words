# k-most-common-words
This program prints the most common words found in a given text.  The problem was originally proposed by John Bently [here](https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf).  As discussed in [programming pearls](https://www.cs.tufts.edu/~nr/cs257/archive/don-knuth/pearls-2.pdf), if two or more words share the same frequency, then rank the selection by alphabetically.
# example
To print the 10 most common words found in the bible:
```
./k-most-common-words 10 bible.txt
```
