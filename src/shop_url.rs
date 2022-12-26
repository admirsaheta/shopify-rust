use once_cell::sync::Lazy;
use regex::Regex;

use crate::ShopifyApp;

// This code defines a static variable named SHOP_REG of type Lazy<Regex> that is initialized with a regular expression using the Lazy::new function. The regular expression is used to match strings that represent a valid Shopify shop URL.

// The Lazy type is a wrapper type provided by the once_cell crate that allows for the initialization of a value to be deferred until it is actually needed. This can be useful for optimizing the startup time of a program by avoiding the cost of initializing values that may not be used.

// The SHOP_REG static variable is then used in an implementation of the ShopifyApp struct. The shopify_url method takes a string slice &str as an argument and returns a bool indicating whether the string matches the regular expression stored in SHOP_REG.

// The regular expression itself is constructed using the Regex::new function from the regex crate. It specifies a pattern that matches strings that start with one or more alphanumeric characters, followed by zero or more hyphens, followed by .myshopify.com. The r"^[a-zA-Z0-9][a-zA-Z0-9\-]*\.myshopify\.com$" string is a raw string literal, which allows for the use of backslashes to escape special characters in the regular expression. 

// The unwrap method is called on the result of Regex::new to unwrap the Result type that it returns, which will panic if the regular expression is invalid.

const SHOP_PATTERN: &str = r"^[a-zA-Z0-9][a-zA-Z0-9\-]*\.myshopify\.com$";

static SHOP_REG: Lazy<Regex> = Lazy::new(|| Regex::new(SHOP_PATTERN).unwrap());

impl ShopifyApp {
    /// This method checks whether the given shop domain string is a valid Shopify shop domain.
    ///
    /// The method takes a shop domain string `&str` as an argument and returns a `bool`.
    pub fn shopify_url(shop: &str) -> bool {
        SHOP_REG.is_match(shop)
    }
}