On to the meagre justifications
for the crimes against humanity
the code in this repo will certainly commit.

# Naming

Do you know how many puns around “doc” etc. are already taken
for something concerning documents or documentation?
Originally, I wanted “[docshelter](http://www.gdcamerica.com/index-8.php)”
but some paper pushers already took it.
I also liked “docsidize”
but “[doxidize](https://twitter.com/Gankro/status/985705992133410817)”
was already taken.
And so on and on and on …

Therefore, I elected to choose the name
whose only other common usage
seems to be in reference to “Dr. Who”-themed cocktails.
I figured we would not be at odds with another but could become good neighbours.

Anyway, I still reserve the rights to the following names:
- docbed
- docpillow
- doccermom

(IANAL but I'm pretty sure it does ~~not~~ work that way.)

# General idea

This software is supposed to manage documents
in the case these documents cannot be prepared sensibly themselves.
The major task in this endeavour is the management of metadata.

## Blobs

Documents are assumed to be immutable blobs
without any inner structure
bestowed upon us by outside forces beyond our control.
In particular, they are without machine-readable metadata.

## Data

In case (part of) the actual data of the document is available in other formats,
this should be modelled in this tool as well.
Currently, no such mechanism has been proposed (by me to me).

## Metadata

Metadata will be stored in a kind of key-value tagging system.

**Example:**
Some file might have the tag `composer:Nikolai Rimsky-Korsakov`.
We might assume that it contains
in some way (audio file, image of sheet music, …)
a musical piece by this composer.

The actual usage of tags and their meanings are up to the users.
In the above example,
the tag might actually mean that this file was created on March 18.

Multiple tags on a document can have the same key.

**Example:**
The transcript of a meeting might be tagged
`topic:acquisition of members, topic:communication problems, topic:open house`
to tell us that all three of these topics were discussed.
For details we would have to open the file using suitable external software.

## Meta²data

Metadata about tags might be interesting,
e.g. that the tag `meeting no:347` was added on 2018-08-30 by j-k.
In a similar vein,
distributed version control seems like a good idea.
Actionable plans have been offered for neither so far.

# Typing

Obviously, any sensible tag key will have a type associated with it
that the values should adhere to,
for example after an `author:` key,
we might want an identifier for a person.
To achieve such a goal, various approaches might be viable.

## Strings

Most things can be modelled as strings.
A quick and dirty approach could thus model a tag as a `(String, String)` pair.

## Strong typing

Employing metaprogramming or a language that has type variables
would allow the usage of established type systems for our own goals.
This seems like a clean and desirable solution
but I am not well-versed enough in any such technology
to seriously consider it for the first draft.

## Hacky ad-hoc horror

Implementing a custom run-time type system might be an option as well.

# Identifiers

I plan on simply using UUIDs for identifying documents
since using established mechanisms beats creativity any time.
