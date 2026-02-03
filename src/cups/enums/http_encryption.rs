
// C representation of the HttpEncryptionT enum
// Original source: /usr/include/cups/http.h

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HttpEncryption {
    IfRequested = 0, // Encrypt if requested (TLS upgrade)
    Never = 1,       // Never encrypt
    Required = 2,    // Encryption is required (TLS upgrade)
    Always = 3,      // Always encrypt (SSL)
}