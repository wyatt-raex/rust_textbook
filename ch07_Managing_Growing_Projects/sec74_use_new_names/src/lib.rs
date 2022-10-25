/*
In the case of importing two items with the same name you can go about it in two ways.

Firstly you can import only the parents and then specify which item you want to use with
its parent through the `::` operator:

`
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {}
    fn function2() -> io::Result<()> {}
`

Or as displayed below you can rename one of the items instead useing the `as` keyword.
`
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {}
    fn function2() -> IoResult<()> {}
`
*/
