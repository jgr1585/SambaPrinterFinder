//! IPP status codes.
//! Original file /usr/include/cups/ipp.h

use std::cmp::Ordering;

#[repr(i32)]
#[derive(Eq, Debug, Clone, Copy, PartialEq, Ord)]
#[allow(dead_code)]
pub enum IppStatus {
    CupsInvalid = -1,                         // Invalid status name for @link ippErrorValue@
    Ok = 0x0000,                              // successful-ok
    OkIgnoredOrSubstituted,                   // successful-ok-ignored-or-substituted-attributes
    OkConflicting,                            // successful-ok-conflicting-attributes
    OkIgnoredSubscriptions,                   // successful-ok-ignored-subscriptions
    OkIgnoredNotifications,                   // successful-ok-ignored-notifications @private@
    OkTooManyEvents,                          // successful-ok-too-many-events
    OkButCancelSubscription,                  // successful-ok-but-cancel-subscription @private@
    OkEventsComplete,                         // successful-ok-events-complete
    RedirectionOtherSite = 0x0200,            // redirection-other-site @private@
    CupsSeeOther = 0x0280,                    // cups-see-other @private@
    ErrorBadRequest = 0x0400,                 // client-error-bad-request
    ErrorForbidden,                           // client-error-forbidden
    ErrorNotAuthenticated,                    // client-error-not-authenticated
    ErrorNotAuthorized,                       // client-error-not-authorized
    ErrorNotPossible,                         // client-error-not-possible
    ErrorTimeout,                             // client-error-timeout
    ErrorNotFound,                            // client-error-not-found
    ErrorGone,                                // client-error-gone
    ErrorRequestEntity,                       // client-error-request-entity-too-large
    ErrorRequestValue,                        // client-error-request-value-too-long
    ErrorDocumentFormatNotSupported,          // client-error-document-format-not-supported
    ErrorAttributesOrValues,                  // client-error-attributes-or-values-not-supported
    ErrorUriScheme,                           // client-error-uri-scheme-not-supported
    ErrorCharset,                             // client-error-charset-not-supported
    ErrorConflicting,                         // client-error-conflicting-attributes
    ErrorCompressionNotSupported,             // client-error-compression-not-supported
    ErrorCompressionError,                    // client-error-compression-error
    ErrorDocumentFormatError,                 // client-error-document-format-error
    ErrorDocumentAccess,                      // client-error-document-access-error
    ErrorAttributesNotSettable,               // client-error-attributes-not-settable
    ErrorIgnoredAllSubscriptions,             // client-error-ignored-all-subscriptions
    ErrorTooManySubscriptions,                // client-error-too-many-subscriptions
    ErrorIgnoredAllNotifications,             // client-error-ignored-all-notifications @private@
    ErrorPrintSupportFileNotFound,            // client-error-print-support-file-not-found @private@
    ErrorDocumentPassword,                    // client-error-document-password-error
    ErrorDocumentPermission,                  // client-error-document-permission-error
    ErrorDocumentSecurity,                    // client-error-document-security-error
    ErrorDocumentUnprintable,                 // client-error-document-unprintable-error
    ErrorAccountInfoNeeded = 0x049C,          // client-error-account-info-needed
    ErrorAccountClosed,                       // client-error-account-closed
    ErrorAccountLimitReached,                 // client-error-account-limit-reached
    ErrorAccountAuthorizationFailed,          // client-error-account-authorization-failed
    ErrorNotFetchable,                        // client-error-not-fetchable
    ErrorInternal = 0x0500,                   // server-error-internal-error
    ErrorOperationNotSupported,               // server-error-operation-not-supported
    ErrorServiceUnavailable,                  // server-error-service-unavailable
    ErrorVersionNotSupported,                 // server-error-version-not-supported
    ErrorDevice,                              // server-error-device-error
    ErrorTemporary,                           // server-error-temporary-error
    ErrorNotAcceptingJobs,                    // server-error-not-accepting-jobs
    ErrorBusy,                                // server-error-busy
    ErrorJobCanceled,                         // server-error-job-canceled
    ErrorMultipleJobsNotSupported,            // server-error-multiple-document-jobs-not-supported
    ErrorPrinterIsDeactivated,                // server-error-printer-is-deactivated
    ErrorTooManyJobs,                         // server-error-too-many-jobs
    ErrorTooManyDocuments,                    // server-error-too-many-documents
    ErrorCupsAuthenticationCanceled = 0x1000, // cups-authentication-canceled - Authentication canceled by user @since CUPS 1.5/macOS 10.7@
    ErrorCupsPki,                             // cups-pki-error - Error negotiating a secure connection @since CUPS 1.5/macOS 10.7@
    ErrorCupsUpgradeRequired,                 // cups-upgrade-required - TLS upgrade required @since CUPS 1.5/macOS 10.7@
}

impl PartialOrd for IppStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}