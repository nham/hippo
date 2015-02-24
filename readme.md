Hippo is a command-line program used to schedule the review of items using spaced repetition. Like other spaced-repetition software (Anki, Mnemosyne), the scheduling algorithm is based on the SM-2 algorithm. Unlike other spaced-repetition software, this is not flashcard-based. An "item" in hippo is just a description of the thing you want to review. The actual information to be reviewed is assumed to be elsewhere (in a text file somewhere, or in some note-taking software, or written down in a notebook, or maybe carved into clay tablets).

# Usage

Add an item with

    hippo add <description>

You can edit an item's description with

    hippo edit <id> <description>

or view more an item's scheduling details by

    hippo view <id>

(the above details won't make much sense unless you understand the SM-2 algorithm) or delete an item with

    hippo remove <id>

To review items, use

    hippo review [<N>]

where <N> is an optional argument for the number of items to review. The default is 20.

You can also list all items with

    hippo list

but this command isn't currently super useful and I probably need to rethink it.
