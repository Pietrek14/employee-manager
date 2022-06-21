# employee-manager

Employee manager is a simple command line tool that allows you to add, remove, and list employees. It is not actually meant to be used, it is just a programming excercise from [this book](https://doc.rust-lang.org/book/ch08-03-hash-maps.html).

## Syntax

You perform actions on the data by stating *sentences*, which follow the basic syntax resembling English. You always start a sentence with an action, e.g. `add`. Then you specify the person you want to add, e.g. `John`. Then you specify the department, e.g. `to HR`. Unfortunately, you can't use spaces in the arguments, e.g. `Add John Doe to HR` will be an invalid statement.

Some actions also require additional keywords, such as `to` or `from`.

## Example sentences

 - `Add John to HR`
 - `Remove Sally from Sales`
 - `List employees from Engineering`