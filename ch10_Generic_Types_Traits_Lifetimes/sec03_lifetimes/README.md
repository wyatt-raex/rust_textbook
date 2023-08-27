# Changes going forward
I'm coming back to this after quite after nearly a year and going to change how I organize the
project.

From now on each section will be a folder containing a `README.md` that will contain my notes on
said section and main code snippets will be located in folders within the section folder. This should
make it so that notes and code are separated and I don't end up with commented out code that says
"this is an example of code that doesn't compile and is the reason we do X."

TL;DR this should make the project easier to understand and read.

# Notes Begin:

Lifetimes: the scope which a reference is valid.
    - are another type of generic that we've already been using.
    - instead of ensuring that a type has the behavior we want, lifetimes ensure references are
    valid as long as we need them to be.

## Preventing Dangling References
