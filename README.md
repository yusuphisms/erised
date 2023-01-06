# erised
Mirroring data

# The Problem
We want to mimic the _makeup_ of a source data to a destination dataset without copying the data 1:1.

# Constraints
1. Respect Cardinality
1. Respect Value Distribution (proporitionality)
1. Respect Relational Data
1. Respect Memory Constraints

# Random Notes So Far
I need to get to writing but don't always know how I want to organize the writing so this section will kind of be random thoughts as they pop up that I want to make sure I note down.

I started this journey by thinking of this problem as a `cardinality` problem: that is, given a dataset estimate the "makeup" of the data by counting how many distinct different types of values there are. Certainly for one type of problem, that is sufficient. If I know rough cardinality, then I write a generator to create a set that matches that cardinality. That led me to discovering `HyperLogLog` and some incredible implementations:
1. [Redis' HyperLogLog data type](https://redis.io/docs/data-types/hyperloglogs/)
1. Actually you can track all the random repositories I am finding on GitHub in my [fancy GitHub list](https://github.com/stars/yusuphisms/lists/cardinality).

But when I spoke with someone else about it and tried to expand a little more on what I'm trying to do, it became apparent that I wasn't interested in just data cardinality -- my problem space was a little wider:
1. Data distribution or proportionality of a value also mattered. As a simple example, cardinality = 3, but for values A,B,C distribution could be 50,25,25 of the whole dataset.
1. Relational Data -- I don't work with data often but I was suprised at how well relational data can conceal itself as not such. Maybe it's because we were using NoSQL data store specifically, but certainly the data, especially across tables but even within the same table, had unspoken relations (read: entanglements??). Counting cardinality alone would lose the relational context.
1. Size of data -- While not astronomical, it was enough that my sweet, sweet laptop cannot hold it in memory. That being said, I also think there is an upper bound for how much data I care to work with but I haven't drawn a line in the sand just yet.
