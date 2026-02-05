
// IPP operation codes as per RFC 8010 and CUPS extensions.
// Original from /usr/include/cups/ipp.h

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IppOp {
    CupsInvalid = -1,                               // Invalid operation name for @link ippOpValue@
    CupsNone = 0,                                   // No operation @private@
    PrintJob = 0x0002,                              // Print-Job: Print a single file
    #[doc(hidden)] PrintUri,                        // Print-URI: Print a single URL @exclude
    ValidateJob,                                    // Validate-Job: Validate job values prior to submission
    CreateJob,                                      // Create-Job: Create an empty print job
    SendDocument,                                   // Send-Document: Add a file to a job
    SendUri,                                        // Send-URI: Add a URL to a job
    CancelJob,                                      // Cancel-Job: Cancel a job
    GetJobAttributes,                               // Get-Job-Attribute: Get information about
    GetJobs,                                        // Get-Jobs: Get a list of jobs
    GetPrinterAttributes,                           // Get-Printer-Attributes: Get information about a printer
    HoldJob,                                        // Hold-Job: Hold a job for printing
    ReleaseJob,                                     // Release-Job: Release a job for printing
    #[deprecated] RestartJob,                       // Restart-Job: Reprint a job @deprecated@
    PausePrinter = 0x0010,                          // Pause-Printer: Stop a printer
    ResumePrinter,                                  // Resume-Printer: Start a printer
    #[doc(hidden)] #[deprecated] PurgeJobs,         // Purge-Jobs: Delete all jobs @deprecated@ @exclude all@
    SetPrinterAttributes,                           // Set-Printer-Attributes: Set printer values
    SetJobAttributes,                               // Set-Job-Attributes: Set job values
    GetPrinterSupportedValues,                      // Get-Printer-Supported-Values: Get supported values
    CreatePrinterSubscriptions,                     // Create-Printer-Subscriptions: Create one or more printer subscriptions @since CUPS 1.2/macOS 10.5@
    CreateJobSubscriptions,                         // Create-Job-Subscriptions: Create one of more job subscriptions @since CUPS 1.2/macOS 10.5@
    GetSubscriptionAttributes,                      // Get-Subscription-Attributes: Get subscription information @since CUPS 1.2/macOS 10.5@
    GetSubscriptions,                               // Get-Subscriptions: Get list of subscriptions @since CUPS 1.2/macOS 10.5@
    RenewSubscription,                              // Renew-Subscription: Renew a printer subscription @since CUPS 1.2/macOS 10.5@
    CancelSubscription,                             // Cancel-Subscription: Cancel a subscription @since CUPS 1.2/macOS 10.5@
    GetNotifications,                               // Get-Notifications: Get notification events @since CUPS 1.2/macOS 10.5@
    #[doc(hidden)] SendNotifications,               // Send-Notifications: Send notification events @private@
    #[doc(hidden)] GetResourceAttributes,           // Get-Resource-Attributes: Get resource information @private@
    #[doc(hidden)] #[deprecated] GetResourceData,   // Get-Resource-Data: Get resource data @private@ @deprecated@
    #[doc(hidden)] GetResources,                    // Get-Resources: Get list of resources @private@
    #[doc(hidden)] GetPrintSupportFiles,            // Get-Printer-Support-Files: Get printer support files @private@
    EnablePrinter,                                  // Enable-Printer: Accept new jobs for a printer
    DisablePrinter,                                 // Disable-Printer: Reject new jobs for a printer
    PausePrinterAfterCurrentJob,                    // Pause-Printer-After-Current-Job: Stop printer after the current job
    HoldNewJobs,                                    // Hold-New-Jobs: Hold new jobs
    ReleaseHeldNewJobs,                             // Release-Held-New-Jobs: Release
    #[doc(hidden)] #[deprecated] DeactivatePrinter, // Deactivate-Printer: Stop a printer and do not accept jobs @deprecated@ @exclude all@
    #[doc(hidden)] #[deprecated] ActivatePrinter,   // Activate-Printer: Start a printer and accept jobs @deprecated@ @exclude all@
    #[doc(hidden)] RestartPrinter,                  // Restart-Printer: Restart a printer @exclude all@
    #[doc(hidden)] ShutdownPrinter,                 // Shutdown-Printer: Turn a printer off @exclude all@
    #[doc(hidden)] StartupPrinter,                  // Startup-Printer: Turn a printer on @exclude all@
    #[doc(hidden)] #[deprecated] ReprocessJob,      // Reprocess-Job: Reprint a job @deprecated@ @exclude all@
    CancelCurrentJob,                               // Cancel-Current-Job: Cancel the current job
    SuspendCurrentJob,                              // Suspend-Current-Job: Suspend the current job
    ResumeJob,                                      // Resume-Job: Resume the current job
    PromoteJob,                                     // Promote-Job: Promote a job to print sooner
    ScheduleJobAfter,                               // Schedule-Job-After: Schedule a job to print after another
    #[doc(hidden)] CancelDocument = 0x0033,         // Cancel-Document: Cancel a document @exclude all@
    #[doc(hidden)] GetDocumentAttributes,           // Get-Document-Attributes: Get document information @exclude all@
    #[doc(hidden)] GetDocuments,                    // Get-Documents: Get a list of documents in a job @exclude all@
    #[doc(hidden)] #[deprecated] DeleteDocument,    // Delete-Document: Delete a document @deprecated@  @exclude all@
    #[doc(hidden)] SetDocumentAttributes,           // Set-Document-Attributes: Set document values @exclude all@
    CancelJobs,                                     // Cancel-Jobs: Cancel all jobs (administrative)
    CancelMyJobs,                                   // Cancel-My-Jobs: Cancel a user's jobs
    #[doc(hidden)] ResubmitJob,                     // Resubmit-Job: Copy and reprint a job @exclude all@
    CloseJob,                                       // Close-Job: Close a job and start printing
    IdentifyPrinter,                                // Identify-Printer: Make the printer beep, flash, or display a message for identification
    #[doc(hidden)] ValidateDocument,                // Validate-Document: Validate document values prior to submission @exclude all@
    #[doc(hidden)] AddDocumentImages,               // Add-Document-Images: Add image(s) from the specified scanner source @exclude all@
    #[doc(hidden)] AcknowledgeDocument,             // Acknowledge-Document: Acknowledge processing of a document @exclude all@
    #[doc(hidden)] AcknowledgeIdentifyPrinter,      // Acknowledge-Identify-Printer: Acknowledge action on an Identify-Printer request @exclude all@
    #[doc(hidden)] AcknowledgeJob,                  // Acknowledge-Job: Acknowledge processing of a job @exclude all@
    #[doc(hidden)] FetchDocument,                   // Fetch-Document: Fetch a document for processing @exclude all@
    #[doc(hidden)] FetchJob,                        // Fetch-Job: Fetch a job for processing @exclude all@
    #[doc(hidden)] GetOutputDeviceAttributes,       // Get-Output-Device-Attributes: Get printer information for a specific output device @exclude all@
    #[doc(hidden)] UpdateActiveJobs,                // Update-Active-Jobs: Update the list of active jobs that a proxy has processed @exclude all@
    #[doc(hidden)] DeregisterOutputDevice,          // Deregister-Output-Device: Remove an output device @exclude all@
    #[doc(hidden)] UpdateDocumentStatus,            // Update-Document-Status: Update document values @exclude all@
    #[doc(hidden)] UpdateJobStatus,                 // Update-Job-Status: Update job values @exclude all@
    #[doc(hidden)] UpdateOutputDeviceAttributes,    // Update-Output-Device-Attributes: Update output device values @exclude all@
    #[doc(hidden)] GetNextDocumentData,             // Get-Next-Document-Data: Scan more document data @exclude all@
    AllocatePrinterResources,                       // Allocate-Printer-Resources: Use resources for a printer.
    CreatePrinter,                                  // Create-Printer: Create a new service.
    DeallocatePrinterResources,                     // Deallocate-Printer-Resources: Stop using resources for a printer.
    DeletePrinter,                                  // Delete-Printer: Delete an existing service.
    GetPrinters,                                    // Get-Printers: Get a list of services.
    ShutdownOnePrinter,                             // Shutdown-One-Printer: Shutdown a service.
    StartupOnePrinter,                              // Startup-One-Printer: Start a service.
    CancelResource,                                 // Cancel-Resource: Uninstall a resource.
    CreateResource,                                 // Create-Resource: Create a new (empty) resource.
    InstallResource,                                // Install-Resource: Install a resource.
    SendResourceData,                               // Send-Resource-Data: Upload the data for a resource.
    SetResourceAttributes,                          // Set-Resource-Attributes: Set resource object  attributes.
    CreateResourceSubscriptions,                    // Create-Resource-Subscriptions: Create event subscriptions for a resource.
    CreateSystemSubscriptions,                      // Create-System-Subscriptions: Create event subscriptions for a system.
    DisableAllPrinters,                             // Disable-All-Printers: Stop accepting new jobs on all services.
    EnableAllPrinters,                              // Enable-All-Printers: Start accepting new jobs on all services.
    GetSystemAttributes,                            // Get-System-Attributes: Get system object attributes.
    GetSystemSupportedValues,                       // Get-System-Supported-Values: Get supported values for system object attributes.
    PauseAllPrinters,                               // Pause-All-Printers: Stop all services immediately.
    PauseAllPrintersAfterCurrentJob,                // Pause-All-Printers-After-Current-Job: Stop all services after processing the current jobs.
    RegisterOutputDevice,                           // Register-Output-Device: Register a remote service.
    RestartSystem,                                  // Restart-System: Restart all services.
    ResumeAllPrinters,                              // Resume-All-Printers: Start job processing on all services.
    SetSystemAttributes,                            // Set-System-Attributes: Set system object attributes.
    ShutdownAllPrinters,                            // Shutdown-All-Printers: Shutdown all services.
    StartupAllPrinters,                             // Startup-All-Printers: Startup all services.
    #[doc(hidden)] Private = 0x4000,                // Reserved @private@
    CupsGetDefault,                                 // CUPS-Get-Default: Get the default printer
    CupsGetPrinters,                                // CUPS-Get-Printers: Get a list of printers and/or classes
    CupsAddModifyPrinter,                           // CUPS-Add-Modify-Printer: Add or modify a printer
    CupsDeletePrinter,                              // CUPS-Delete-Printer: Delete a printer
    #[doc(hidden)] #[deprecated] CupsGetClasses,    // CUPS-Get-Classes: Get a list of classes @deprecated@ @exclude all@
    CupsAddModifyClass,                             // CUPS-Add-Modify-Class: Add or modify a class
    CupsDeleteClass,                                // CUPS-Delete-Class: Delete a class
    #[doc(hidden)] CupsAcceptJobs,                  // CUPS-Accept-Jobs: Accept new jobs on a printer @exclude all@
    #[doc(hidden)] CupsRejectJobs,                  // CUPS-Reject-Jobs: Reject new jobs on a printer @exclude all@
    #[deprecated] CupsSetDefault,                   // CUPS-Set-Default: Set the default printer
    #[deprecated] CupsGetDevices,                   // CUPS-Get-Devices: Get a list of supported devices @deprecated@
    #[deprecated] CupsGetPpds,                      // CUPS-Get-PPDs: Get a list of supported drivers @deprecated@
    CupsMoveJob,                                    // CUPS-Move-Job: Move a job to a different printer
    CupsAuthenticateJob,                            // CUPS-Authenticate-Job: Authenticate a job @since CUPS 1.2/macOS 10.5@
    #[deprecated] CupsGetPpd,                       // CUPS-Get-PPD: Get a PPD file @deprecated@
    CupsGetDocument = 0x4027,                       // CUPS-Get-Document: Get a document file @since CUPS 1.4/macOS 10.6@
    CupsCreateLocalPrinter                          // CUPS-Create-Local-Printer: Create a local (temporary) printer @since CUPS 2.2@
}