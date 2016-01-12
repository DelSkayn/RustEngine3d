# Speed
This document is ment as a log for the speed of the engine tracking the whether features are fast enough.
Whenever any testing is done on the speed of certain parts the results should be found here.

## How to test
TODO: write something about the profile lib once done.

## Tests

### Message passing
The Massage passing proved to be fast enough for my liking.
The current delay when writing a message is under 0.1 ms. 
This test was conducded with the main loop in between.
However is did saw some problems where events might stack up if the main thread is running really fast.
Something that should not be a problem when it actually starts doing some work.
