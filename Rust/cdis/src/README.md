When it comes down to error handling i decided to use the bad-practice
String implementation here. On purpose. And here is why: Typically in
Rust (when you implement error handling the right way) you will define
an own custom error type (an enum implementing the Error trait). That
custom error type should also implement the From trait for every error
type you want to wrap into your custom error type, to transport cause
and source of the underlying error and to also make comfortable use of
the ? operator. In short: Error handling in Rust (if done right) will
come with some effort. Too much effort for this tiny application here
and too complex for myself, when i need to remember all that stuff in
half a year or so, since i do not program in Rust on a daily base. The
reason why error handling in Rust should be done better than using the
sloppy String implementation or using Box<dyn Error> (which also will
not help much here and also is not a great option in general, cause of
its dynamic nature) is this: Usually you want your logic functions to
return some error with own custom error data (like a String describing
the error) while at the same time not loosing the occurred underlying
original error (like i.e. an I/O error), to preserve all the existing
error informations for later use. In example to log them in detail, in
the main() function, with the occured error´s stack information still
available. Therefore just using a String (as error type) is considered
as bad-practice, cause you loose all that information. Another option
is to use one of the well-known error handling crates out there, like
thiserror, anyhow, or SNAFU. But in my opinion this will just move the
steep learning curve from Rust´s built-in error handling towards some
"how to use that crate" effort. Also this seems a bit overdosed to me,
for such a tiny application. And since i do not need to achieve above
goal of "preserving all error informations" at the moment and the use
of Box<dyn Error> would not help much either, i decided it is legit to
be "no compromise" and to stick to the sloppiest but simplest approach.

If you are looking for a great tutorial about error handling in Rust,
you should read this: https://blog.burntsushi.net/rust-error-handling
I invested a good amount of time into error handling in Rust and imo
Andrew Gallant's blog post is by far the best read you can have here.

I also decided against the result type alias idiom, typically used
together with some own defined custom error type. Since there are
often huge time periods between my Rust coding moments, it is more
clear to me then, what is going on, without using all such things
i have to remember to, like in example the result type alias idiom.
