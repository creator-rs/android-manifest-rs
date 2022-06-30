use super::resources::*;
use serde::{Deserialize, Serialize};

/// A name-value pair for an item of additional, arbitrary data that can be
/// supplied to the parent component.
///
/// A component element can contain any number
/// of `<meta-data>` subelements. The values from all of them are collected in a
/// single [`Bundle`] object and made available to the component as the
/// [`PackageItemInfo.metaData`] field.
///
/// Ordinary values are specified through the [`value`] attribute. However, to assign a
/// resource ID as the value, use the [`resource`] attribute instead. For example, the
/// following code assigns whatever value is stored in the @string/kangaroo resource to
/// the `"zoo"` name:
///
/// ## XML Examples
///
/// ```xml
/// <meta-data android:name="zoo" android:value="@string/kangaroo" />
/// ```
/// On the other hand, using the resource attribute would assign `"zoo"` the
/// numeric ID of the resource, not the value stored in the resource:
///
/// ```xml
/// <meta-data android:name="zoo" android:resource="@string/kangaroo" />
/// ```
///
/// It is highly recommended that you avoid supplying related data as multiple
/// separate `<meta-data>` entries. Instead, if you have complex data to
/// associate with a component, store it as a `resource` and use the resource
/// attribute to inform the component of its ID.
///
/// ## XML Syntax
/// ```xml
/// <meta-data android:name="string"
///            android:resource="resource specification"
///            android:value="string" />
/// ```
///
/// ## Contained in
/// * [`<activity>`]
/// * [`<activity-alias>`]
/// * [`<applocation>`]
/// * [`<service>`]
/// * [`<receiver>`]
/// * [`<provider>`]
///
/// ## Introduced in
/// API Level 1
///
/// [`Bundle`]: https://developer.android.com/reference/android/os/Bundle
/// [`PackageItemInfo.metaData`]: https://developer.android.com/reference/android/content/pm/PackageItemInfo#metaData
/// [`value`]: crate::MetaData#structfield.value
/// [`resource`]: crate::MetaData#structfield.resource
/// [`<activity>`]: crate::Activity
/// [`<activity-alias>`]: crate::ActivityAlias
/// [`<applocation>`]: crate::Application
/// [`<service>`]: crate::Service
/// [`<receiver>`]: crate::Receiver
/// [`<provider>`]: crate::Provider
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone)]
pub struct MetaData {
    /// A unique name for the item. To ensure that the name is unique, use a Java-style
    /// naming convention — for example, `"com.example.project.activity.fred"`.
    #[yaserde(attribute, prefix = "android")]
    pub name: Option<String>,
    /// A reference to a resource. The ID of the resource is the value assigned to the
    /// item. The ID can be retrieved from the meta-data Bundle by the
    /// [`Bundle.getInt()`] method.
    ///
    /// [`Bundle.getInt()`]: https://developer.android.com/reference/android/os/BaseBundle#getInt(java.lang.String)
    #[yaserde(attribute, prefix = "android")]
    pub resource: Option<AnyResource>,
    /// The value assigned to the item. The data types that can be assigned as values and
    /// the Bundle methods that components use to retrieve those values are listed in the
    /// following table: https://developer.android.com/guide/topics/manifest/meta-data-element#val
    #[yaserde(attribute, prefix = "android")]
    pub value: Option<String>,
}
