#!/bin/bash

# Get the Facebook address by deriving the PDA and getting the first line (head -n1)
facebookAddress=$(flare generate-pda --program DMUK1PVbLRrX8eK63K7F4jt31nnAQ5Vp8mgs7mnrbQNn self-custodial-facebook2,$1 | head -n1)

# Read the contents of that account
flare read-account --program DMUK1PVbLRrX8eK63K7F4jt31nnAQ5Vp8mgs7mnrbQNn --account $facebookAddress
