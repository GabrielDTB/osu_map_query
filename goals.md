Develop as a library for parsing and working with maps.

Ideally, build to be modular. That is, it should be possible to only query a specific
subset of features. This would increase performance for all applications. 
At the same time, provide a few reasonable default profiles.

Build in concurrency. Decide whether threaded or just coroutines would be best.
To facilitate the use of concurrency, batch readings must be easy to perform.

Cover every edge case possible.

Practice good code segregation.

Find a good compromise between readability, speed, and edge case coverage.

Parse strings by discarding any non-character tokens.