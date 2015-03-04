Hippo is a command-line program used to schedule the review of items using spaced repetition. Like other spaced-repetition software (Anki, Mnemosyne), the scheduling algorithm is based on the SM-2 algorithm. Unlike other spaced-repetition software, this is not flashcard-based. An "item" in hippo is just a description of the thing you want to review. The actual information to be reviewed is assumed to be elsewhere (in a text file somewhere, or in some note-taking software, or written down in a notebook, or maybe carved into clay tablets).

# Usage

Add an item with

    hippo add <description>

You can edit an item's description with

    hippo edit <id> <description>

or view more an item's scheduling details by

    hippo view <id>

(the above details won't make much sense unless you understand the scheduling algorithm) or delete an item with

    hippo remove <id>

To review items, use

    hippo review [<N>]

where <N> is an optional argument for the number of items to review. The default is 20.

You can also review a specific item by using the `--id` flag:

    hippo review --id=<id>

You can also list all items with

    hippo list [<string>]

where <string> is an optional search text argument. Only item with descriptions containing the string will be displayed.

# Scheduling algorithm

Each item has four fields that are used to make the review schedule:

  - *last_reviewed*, the timestamp when the item was last reviewed
  - *ff*, the *familarity factor* which is called "easiness factor" in the original SM-2 algorithm. Why did I change it? I dunno, personal taste? What does it matter? Maybe don't worry about it.
  - *iri*, the *inter-repetition interval*. This determines how much time should elapse between reviews of an item in days.
  - *int_step*, mostly a thing for tracking newly learned items.

When a new item is added, the initial values of these fields are:

  - last_reviewed: the time it was created
  - ff: 2.5
  - iri: 1.0
  - int_step: 1

So what happens when you do the `review` command is we first determine which items need to be reviewed by finding out which items have

    (<current timestamp> - last_reviewed) > iri * 86400

(since last_reviewed is stored as a timestamp). Once we have the list, we sort the items to be reviewed so that they are in order "most urgently in need of review" to "least urgently in need of review". This is done by checking how much `(current time - last review time)` exceeds the IRI in seconds.

Each item presents the description. You're supposed to review the material, and then rate on a scale of 0-5 how familiar the item feels, with 5 being most familiar and 0 being least. So if you think you have a great grasp of the item, rate it a 4 or a 5 and the scheduler will present that item less often. If you feel a bit shaky but still understand it, give is a 2 or a 3. If you totally forgot it, rate it 0 or 1. The significance of these is explained below.

If we let `fam` be the 0-5 rating I've just described above, the logic for updating the 4 fields is given by:

    if fam < 2:
        int_step <-- 1
    else:
        int_step <-- int_step + 1

    if int_step == 1:
        iri <-- 1.0
    else if int_step == 2:
        iri <-- 3.0
    else:
        iri <-- iri + FF

    if fam == 2:
        FF <-- FF - 0.32
    else if fam == 3:
        FF <-- FF - 0.14
    else if fam == 5:
        FF <-- FF + 0.1

    if FF < 1.3:
        FF <-- 1.3

    last_reviewed <-- current time

So for `fam` of 0 or 1, the int_step gets reset and it's like we are re-learning the item anew. Note that we also do not adjust the FF in this case, since FF only matters for items of int_step > 2.

If `fam >= 2`, we update the familiarity factor. In the case of `fam = 2 or 3`, the FF gets adjusted downward `fam == 4` results in no adjustment, while `fam == 5` increases familiarity.

There's also logic to ensure that FF never goes below 1.3 (this is in the original SM-2 algorithm and I've imported it unthinkingly. I haven't had the chance to test it yet).
