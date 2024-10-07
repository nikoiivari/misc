
# Define the capabilities the program needs. These will be stored in the header
# of the executable format. Besides memory allocation there are others to ask for.
ask @[8:]       # ask for eight pages of accumulator
req @[4:]       # require a minimum of four pages of accumulator to run
req $[1/4:]     # require 1/4th of a page of cache to run (256/4 = 64)

# Link in functions from other files or libraries
# use thingamagadget.asm

# Variable definitions in the scope of func main. These are similiar to labels,
# and have a colon ':' after the variable name.
var accum   : even  .stuvwxyz   8byte   capability
var cache   : odd   .stuvwxyz   8byte   capability
var foo     : even  .stuv       4byte   unsigned
var bar     : pack      .wxyz   4byte   unsigned    # .wxyz gets packed into
                                                    # the previous dword.
                                                    # It will be even.
in argc     : odd   .stuvwxyz   8byte   unsigned
in argv     : even  .stuvwxyz   8byte   capability

out status  : odd   .z          1byte   signed

# main is a label, but the fun keyword makes it a function label.
fun main (argc, argv):

    accum = @           # save accumulator
    @ = @[:0...3]       # Square brackets '[' and ']' imply indexing -- not direct memory access

return (status)         # fun main ends
