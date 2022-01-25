This is a libray that makes a boolean that "takes" up 1 bit instead of 1 byte.
 
# Examples
```
use bit_bool::OneBitBool;
 
let b = OneBitBool::from(true);
 
assert_eq!(b.get_index(0), true);
```
