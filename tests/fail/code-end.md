<!-- Rule: the text after a code block or an HTML comment is checked, and an indented line that does not follow a blank line is not code. -->

~~~text
code
~~~
After the tilde fence the idiom is load-bearing.

````markdown
```
code
```
````
After the long fence the idiom is load-bearing.

    indented code

After the indented block the idiom is load-bearing.
- A list item that wraps onto
    a continuation line that is load-bearing.

<!-- A comment. -->
After the comment the idiom is load-bearing.
Before <!-- an inline comment --> the idiom is load-bearing.

<!--
A comment on several lines.
--> After the end of the comment the idiom is load-bearing.
