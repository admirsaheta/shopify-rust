use textnonce::TextNonce;

use crate::ShopifyApp;

/// This is an implementation of the `ShopifyApp` struct.
///
/// # Examples
///
/// use shopify_app::ShopifyApp;
///
/// let app = ShopifyApp::new();
/// let nonce = app.new_nonce();
/// let auth_uri = app.new_auth_uri("my-shop.myshopify.com", "http://localhost:3000/auth", &nonce);


    /// This method generates a new nonce value as a string.
    ///
    /// A nonce (number used once) is a random value that is used to protect against replay attacks in authentication systems. This method uses the `TextNonce` type from the `ring` crate to generate a cryptographically secure random value and converts it into a string. The string is then modified by replacing plus signs with hyphens and forward slashes with underscores.
    ///
    /// # Examples
    ///
    /// use shopify_app::ShopifyApp;
    ///
    /// let app = ShopifyApp::new();
    /// let nonce = app.new_nonce();

    /// This method generates a new authorization URI for the Shopify OAuth authentication flow.
    ///
    /// The method takes a shop domain string `&str`, a return URI string `&str`, and a nonce string `&str` as arguments and returns a `String`. The `shop` argument should be the domain of the Shopify shop that the application is being installed on, the `return_uri` argument should be the URI that the user's browser should be redirected to after authentication, and the `nonce` argument should be a nonce value to protect against replay attacks.
    ///
    /// The method uses the `api_key` and `scopes` fields of the `credentials` field of the `ShopifyApp` struct, and the `access_mode` field of the struct, to construct the authorization URI using a format string. The URI is then returned.
    ///
    /// # Examples
    ///
    /// use shopify_app::ShopifyApp;
    ///
    /// let app = ShopifyApp::new();
    /// let nonce = app.new_nonce();
    /// let auth_uri = app.new_auth_uri("my-shop.myshopify.com", "http://localhost:3000/auth", &nonce);

    impl ShopifyApp {
        pub fn new_nonce() -> String {
        TextNonce::new()
           .into_string()
           .replace("+", "-")
           .replace("/", "_")
    }

    pub fn new_auth_uri(&self, shop: &str, return_uri: &str, nonce: &str) -> String {
        format!(
            "https://{shop}/admin/oauth/authorize?client_id={api_key}&scope={scopes}&redirect_uri={redirect_uri}&state={nonce}&grant_options[]={access_mode}",
            shop = shop,
            api_key = self.credentials.api_key,
            scopes=  self.scopes.join(","),
            redirect_uri = return_uri,
            nonce = nonce,
            access_mode = self.access_mode.as_string()
        )
    }
}