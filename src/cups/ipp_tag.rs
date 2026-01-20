// IPP Tag values
// Original file: /usr/include/cups/ipp.h

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IPPTag {
    CupsInvalid = -1,            // Invalid tag name for @link ippTagValue@
    Zero = 0x00,                 // Zero tag - used for separators
    Operation,                   // Operation group
    Job,                         // Job group
    End,                         // End-of-attributes
    Printer,                     // Printer group
    UnsupportedGroup,            // Unsupported attributes group
    Subscription,                // Subscription group
    EventNotification,           // Event group
    Resource,                    // Resource group
    Document,                    // Document group
    System,                      // System group
    UnsupportedValue = 0x10,     // Unsupported value
    Default,                     // Default value
    Unknown,                     // Unknown value
    NoValue,                     // No-value value
    NotSettable = 0x15,          // Not-settable value
    DeleteAttr,                  // Delete-attribute value
    AdminDefine,                 // Admin-defined value
    Integer = 0x21,              // Integer value
    Boolean,                     // Boolean value
    Enum,                        // Enumeration value
    String = 0x30,               // Octet string value
    Date,                        // Date/time value
    Resolution,                  // Resolution value
    Range,                       // Range value
    BeginCollection,             // Beginning of collection value @exclude all@
    TextLang,                    // Text-with-language value
    NameLang,                    // Name-with-language value
    EndCollection,               // End of collection value @exclude all@
    Text = 0x41,                 // Text value
    Name,                        // Name value
    ReservedString,              // Reserved for future string value @private@
    Keyword,                     // Keyword value
    Uri,                         // URI value
    UriScheme,                   // URI scheme value
    Charset,                     // Character set value
    Language,                    // Language value
    MimeType,                    // MIME media type value
    MemberName,                  // Collection member name value @exclude all@
    Extension = 0x7f,            // Extension point for 32-bit tags @exclude all@
    CupsMask = 0x7fffffff,       // Mask for copied attribute values @private@
    CupsConst = -0x7fffffff - 1, // Bitflag for copied/const attribute values @private@
}