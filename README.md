# Lines of Code (loc)
A simple rust binary for getting the lines of code in a specified directory.

### Example
```
loc src
```
> Prints the lines found from all non excluded files in the `./src` directory.

### Installation
Clone from git
```
git clone https://github.com/sqwyer/loc.git loc
```

Navigate to directory
```
cd loc
```

Build binary
```
cargo build --release
```

Install binary
```
cargo install --path .
```

----
Note: this project is not very well made as it's my first ever rust project