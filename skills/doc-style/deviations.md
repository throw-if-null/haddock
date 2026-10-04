# Deviations from ASD-STE100

The rules in [rules.md](rules.md) are based on ASD-STE100, Simplified Technical English. This
file lists each difference between the two.

The STE rule descriptions in this file are not yet verified against the published text of
ASD-STE100. Issue: TODO.

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
| A procedural sentence has a maximum of 20 words. A descriptive sentence has a maximum of 25 words. | Every sentence has a maximum of 25 words. | Changed | TODO |
| A paragraph has a maximum of 6 sentences. | No rule. | Not adopted | TODO |
| A noun cluster has a maximum of 3 words. | No rule. | Not adopted | TODO |
| Use only the approved verb forms. Do not use the `-ing` form as a verb. | No rule. | Not adopted | TODO |
| Use the active voice in procedures. Write an instruction as a command. | Use active voice when it names the component that acts. | Changed | TODO |
| Write one topic per sentence. | State one fact, requirement, or instruction per sentence. | Adopted | |
| Use a vertical list for a sequence of steps. | Use a list for independent requirements or steps. | Adopted | |
| Do not omit articles (`a`, `the`). | No rule. | Not adopted | TODO |
| No equivalent. | Use MUST, MUST NOT, SHOULD, and MAY for requirements. | Added | TODO |
| No equivalent. | Remove idioms, vague qualifiers, filler, anthropomorphism, and promotional words. | Added | TODO |
| No equivalent. | Mark each statement as a fact, a requirement, an assumption, or a recommendation. | Added | TODO |
| No equivalent. | State each fact in one place. | Added | TODO |
