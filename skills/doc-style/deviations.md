# Deviations from ASD-STE100

The rules in [rules.md](rules.md) use ASD-STE100, Simplified Technical English, as a starting
point. They do not implement the standard. This file lists where the two differ, so a reader
who knows STE can see which rules to expect.

The STE column paraphrases the standard. It is not a quotation.

## Status values

| Status | Meaning |
| --- | --- |
| Adopted | `rules.md` contains the STE rule. |
| Changed | `rules.md` contains a modified form of the STE rule. |
| Not adopted | `rules.md` does not contain the STE rule. |
| Added | `rules.md` contains a rule that STE does not have. |

## Rule comparison

| STE rule | `rules.md` | Status | Reason |
| --- | --- | --- | --- |
| Use only words from the STE dictionary, each with its approved meaning. | The controlled vocabulary is not enforced when it conflicts with established software engineering terminology. | Changed | Software engineering terms such as `idempotent` and `transaction` have no approved STE equivalent. |
| A procedural sentence has a maximum of 20 words. A descriptive sentence has a maximum of 25 words. | Every sentence has a maximum of 25 words. | Changed | The checker cannot tell a procedural sentence from a descriptive sentence. One limit keeps the length check mechanical. |
| A paragraph has a maximum of 6 sentences. | No rule. | Not adopted | The structure rules move independent statements into lists and tables. This limits paragraph length without a fixed count. |
| A noun cluster has a maximum of 3 words. | No rule. | Not adopted | Many software engineering terms are longer noun clusters, for example `continuous integration pipeline`. A limit forces a paraphrase of an established term. |
| Use only the approved verb forms. Do not use the `-ing` form as a verb. | No rule. | Not adopted | Software engineering terms use the `-ing` form, for example `logging`, `caching`, and `failing test`. A verb form restriction conflicts with established terminology. |
| Use the active voice in procedures. Write an instruction as a command. | Use active voice when it names the component that acts. | Changed | A description of system behavior sometimes has no relevant actor, for example `The lock file is copied to the output directory`. Active voice is required when it identifies the component that acts. |
| Write one topic per sentence. | State one fact, requirement, or instruction per sentence. | Adopted | |
| Use a vertical list for a sequence of steps. | Use a list for independent requirements or steps. | Adopted | |
| Do not omit articles (`a`, `the`). | No rule. | Not adopted | Under review for a later version. |
| No equivalent. | Use MUST, MUST NOT, SHOULD, and MAY for requirements. | Added | Software specifications use the RFC 2119 keywords to state requirement strength. |
| No equivalent. | Remove idioms, vague qualifiers, filler, anthropomorphism, and promotional words. | Added | STE excludes these words through its dictionary. These rules do not enforce the dictionary, so they name the excluded word classes, and the checker detects them. |
| No equivalent. | Mark each statement as a fact, a requirement, an assumption, or a recommendation. | Added | A technical decision depends on which statements are verified and which are assumed. |
| No equivalent. | State each fact in one place. | Added | A change to one copy of a duplicated fact does not update the other copy. The two copies then contradict each other. |
