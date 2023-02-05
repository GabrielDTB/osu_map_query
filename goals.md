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

### Note Feb 5, 2023:

I have started modularizing the beatmap struct and I'm finding less need for fine-grained modularization. There is only so much modularity that a data-type can have without incurring some penalties. Having the container type small enough while the sub-containers are related within themselves but not with others looks like a good middle ground.