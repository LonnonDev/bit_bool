/*! This is a libray that makes a boolean that "takes" up 1 bit instead of 1 byte.
 
# Examples
```
use bit_bool::OneBitBool;
 
let b = OneBitBool::from(true);
 
assert_eq!(b.get_index(0), true);
```
*/

use std::fmt::Display;

#[test]
fn bit_bool_test() {
    let x = OneBitBool::from(true);
    let mut y = OneBitBool::from(true);
    y.change_index(2, false);
    println!("{} {}", x, y);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OneBitBool {
    bool: u8
}

impl OneBitBool {
    /// Create a new `OneBitBool` from a bool
    /// 
    /// # Examples
    /// ```
    /// use bit_bool::OneBitBool;
    /// 
    /// let b = OneBitBool::from(true);
    /// ```
    pub fn from(bool: bool) -> Self {
        if bool {
            Self { bool: u8::MAX }
        } else {
            Self { bool: 0 }
        }
    }
    /// Empty constructor
    /// 
    /// # Examples
    /// ```
    /// use bit_bool::OneBitBool;
    /// 
    /// let b = OneBitBool::empty();
    /// 
    /// assert_eq!(b, OneBitBool::from(false));
    /// ```
    pub fn empty() -> Self {
        Self { bool: 0 }
    }
    /// Gets the value of the bit at the given index
    /// 
    /// # Examples
    /// ```
    /// use bit_bool::OneBitBool;
    /// 
    /// let b = OneBitBool::from(true);
    /// 
    /// assert_eq!(b.get_index(0), true);
    /// ```
    pub fn get_index(&self, index: usize) -> bool {
        self.bool & (1 << index) != 0
    }
    /// Changes the value of the bit at the given index
    /// 
    /// # Examples
    /// ```
    /// use bit_bool::OneBitBool;
    /// 
    /// let mut b = OneBitBool::from(true);
    /// 
    /// b.change_index(0, false);
    /// ```
    pub fn change_index(&mut self, index: usize, value: bool) {
        if value {
            self.bool |= 1 << index;
        } else {
            self.bool &= !(1 << index);
        }
    }
}

/// A trait for displaying a `OneBitBool`
/// 
/// # Examples
/// ```
/// use bit_bool::OneBitBool;
/// 
/// let b = OneBitBool::from(true);
/// 
/// assert_eq!(format!("{}", b), "11111111");
/// ```
impl Display for OneBitBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.bool)
    }
}