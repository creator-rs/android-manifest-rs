use serde::{Deserialize, Serialize};

/// ## contained in:
/// `<intent-filter>`
///
/// ## description:
/// Adds a data specification to an intent filter. The specification can be just
/// a data type (the mimeType attribute), just a URI, or both a data type and a
/// URI. A URI is specified by separate attributes for each of its parts:
///
/// ```xml
/// <scheme>://<host>:<port>[<path>|<pathPrefix>|<pathPattern>]
/// ```
///
/// ## Examples
///
/// These attributes that specify the URL format are optional, but also mutually
/// dependent:
/// * If a scheme is not specified for the intent filter, all the other URI
///   attributes are ignored.
/// * If a host is not specified for the filter, the port attribute and all the
///   path attributes are ignored.
///
/// All the `<data>` elements contained within the same `<intent-filter>`
/// element contribute to the same filter. So, for example, the following filter
/// specification,
///
/// ```xml
/// <intent-filter ...>
///     <data android:scheme="something" android:host="project.example.com" />
///   ...
/// </intent-filter>
/// ```
/// is equivalent to this one:
/// ```xml
/// <intent-filter ...>
///     <data android:scheme="something" />
///     <data android:host="project.example.com" />
///   ...
/// </intent-filter>
/// ```
/// You can place any number of <data> elements inside an `<intent-filter>` to
/// give it multiple data options. None of its attributes have default values.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "data")]
pub struct Data {
    /// The scheme part of a URI. This is the minimal essential attribute for
    /// specifying a URI; at least one scheme attribute must be set for the
    /// filter, or none of the other URI attributes are meaningful. A scheme
    /// is specified without the trailing colon (for example, http, rather than
    /// http:). If the filter has a data type set (the `mimeType` attribute)
    /// but no scheme, the `content:` and `file:` schemes are assumed.
    ///
    /// ## Note
    ///
    /// Scheme matching in the Android framework is case-sensitive, unlike the
    /// RFC. As a result, you should always specify schemes using lowercase
    /// letters.
    #[serde(rename = "android:scheme")]
    pub scheme: Option<String>,
    /// The host part of a URI authority. This attribute is meaningless unless a
    /// scheme attribute is also specified for the filter. To match multiple
    /// subdomains, use an asterisk (*) to match zero or more characters in the
    /// host. For example, the host `*.google.com` matches `www.google.com`,
    /// `.google.com`, and `developer.google.com.` The asterisk must b`e the
    /// first character of the host attribute. For example, the host
    /// `google.co.` is invalid because the asterisk wildcard is not the first
    /// character.
    ///
    /// `Note:` host name matching in the Android framework is case-sensitive,
    /// unlike the formal RFC. As a result, you should always specify host names
    /// using lowercase letters.
    #[serde(rename = "android:host")]
    pub host: Option<String>,
    /// The port part of a URI authority. This attribute is meaningful only if
    /// the scheme and host attributes are also specified for the filter.
    /// * `android:path`
    /// * `android:pathPrefix`
    /// * `android:pathPattern`
    ///
    /// The path part of a URI which must begin with a /. The path attribute
    /// specifies a complete path that is matched against the complete path in
    /// an Intent object. The `pathPrefix` attribute specifies a partial
    /// path that is matched against only the initial part of the path in the
    /// Intent object. The pathPattern attribute specifies a complete path
    /// that is matched against the complete path in the Intent object, but it
    /// can contain the following wildcards:
    /// * An asterisk `('*')` matches a sequence of 0 to many occurrences of the
    ///   immediately preceding character.
    /// * A period followed by an asterisk `(".*")` matches any sequence of 0 to
    ///   many characters.
    pub port: Option<String>,
    /// The path part of a URI which must begin with a /. The path attribute
    /// specifies a complete path that is matched against the complete path in
    /// an Intent object. The pathPrefix attribute specifies a partial path
    /// that is matched against only the initial part of the path in the Intent
    /// object. The pathPattern attribute specifies a complete path that is
    /// matched against the complete path in the Intent object, but it can
    /// contain the following wildcards: An asterisk `('*')` matches a
    /// sequence of 0 to many occurrences of the immediately preceding
    /// character. A period followed by an asterisk `(".*")` matches any
    /// sequence of 0 to many characters. Because '\' is used as an escape
    /// character when the string is read from XML (before it is parsed as a
    /// pattern), you will need to double-escape: For example, a literal '*'
    /// would be written as `"\\*"` and a literal `'\'` would be written as
    /// `"\\\\"`. This is basically the same as what you would need to write if
    /// constructing the string in Java code. For more information on these
    /// three types of patterns, see the descriptions of `PATTERN_LITERAL`,
    /// `PATTERN_PREFIX`, and `PATTERN_SIMPLE_GLOB` in the PatternMatcher class.
    /// These attributes are meaningful only if the scheme and host attributes
    /// are also specified for the filter.
    #[serde(rename = "android:path")]
    pub path: Option<String>,
    #[serde(rename = "android:pathPattern")]
    pub path_pattern: Option<String>,
    #[serde(rename = "android:pathPrefix")]
    pub path_prefix: Option<String>,
    /// A MIME media type, such as `image/jpeg` or `audio/mpeg4-generic`. The
    /// subtype can be the asterisk wildcard (*) to indicate that any subtype
    /// matches. It's common for an intent filter to declare a `<data>` that
    /// includes only the android:mimeType attribute.
    ///
    /// `Note:` MIME type matching in the Android framework is case-sensitive,
    /// unlike formal RFC MIME types. As a result, you should always specify
    /// MIME types using lowercase letters.
    #[serde(rename = "android:mimeType")]
    pub mime_type: Option<String>,
}
