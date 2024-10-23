
# Define the capabilities the program needs. These will be stored in the header
# of the executable format. Besides memory allocation there are others to ask for.
ask @[8:]       # ask for eight pages of accumulator
req @[4:]       # require a minimum of four pages of accumulator to run
req $[1/4:]     # require 1/4th of a page of cache to run (256/4 = 64)

# Link in functions from other files or libraries
# use thingamagadget.asm
use types           # what would the code look like without types?
use string          # does parsing of argv[] require this?

scope main
    # Variable definitions in the scope of func main. These are similiar to labels,
    # and have a colon ':' after the variable name.
    var accum   : even  .stuvwxyz   8byte   capability
    var cache   : odd   .stuvwxyz   8byte   capability
    var foo     : even  .stuv       4byte   unsigned
    var bar     : pack      .wxyz   4byte   unsigned    # .wxyz gets packed into
                                                        # the previous dword.
                                                        # It will be even.
    in argc     : odd   .stuvwxyz   8byte   signed
    in argv[]   : even  .stuvwxyz   8byte   capability  # argv[] should use string?

    out status  : odd          .z   1byte   signed
epocs

# main is a label, but the fun keyword makes it a function label.
fun main (argc, argv):

    accum = @           # save accumulator
    @ = @[0:0...3]      # Square brackets '[' and ']' imply indexing -- not direct memory access
    @ << 3 b0           # Shift left by 3 bit positions, slide down by 3 registers. Value is now
                        # at the register @[:3].

    @ = accum           # restore accumulator
    @ = @[0:0...15]     # Accumulator region is 16 registers long

    @subdiv 4           # Set sub-division to four registers. All operations on the accumulator
                        # will be replicated on all the current region accumulator registers
                        # in groups of four registers.
    
    status = 0          # Gotta always return a status byte from main. Something has to be written
                        # to status before the function exits.
    
nuf (status)            # fun main ends, returning status.
