use crate::cups::c_interop::{cups_do_request, cups_last_error, cups_last_error_string, cups_server, http_close, http_connect2, ipp_add_string, ipp_delete, ipp_new_request, ipp_port};
use crate::cups::http_encryption::HttpEncryption;
use crate::cups::ipp_operations::IppOp::CupsAddModifyPrinter;
use crate::cups::ipp_status::IppStatus::OkEventsComplete;
use crate::cups::ipp_tag::IPPTag;
use crate::cups::protocol_families::PF;
use url::Url;
use crate::smb::SambaCredentials;

mod c_interop;
mod protocol_families;
mod http_encryption;
mod ipp_operations;
mod ipp_tag;
mod ipp_status;

pub fn connect_to_printer(creds: SambaCredentials, url: Url, printer_name: String) {
    let cups_server = cups_server().unwrap_or(String::new());
    let ipp_port = ipp_port();


    let http_t = http_connect2(
        &cups_server,
        ipp_port,
        None,
        PF::Unspec,
        HttpEncryption::IfRequested,
        true,
        30000,
        None,
    );

    if http_t.expect("REASON").is_null() {
        eprintln!("Failed to connect to CUPS server at {}:{}", cups_server, ipp_port);
        return;
    }

    let request = ipp_new_request(CupsAddModifyPrinter);
    let mut smb_printer_uri = url;

    smb_printer_uri
        .set_username(&*creds.username)
        .expect("Unable to set username");

    smb_printer_uri
        .set_password(Option::from(&*creds.password))
        .expect("Unable to set password");

    ipp_add_string(request, IPPTag::Operation, IPPTag::Uri,
                   Option::from("printer-uri"), None, &*("ipp://localhost/printers/".to_owned() + &printer_name));

    ipp_add_string(request, IPPTag::Printer, IPPTag::Uri,
                    Option::from("device-uri"), None,
                    &smb_printer_uri.to_string());

    // ipp_add_string(request, IPPTag::Printer, IPPTag::Name,
    //                 Option::from("ppd-name"), None, "everywhere");


    let response = cups_do_request(http_t, request, "/admin/");

    if cups_last_error() > OkEventsComplete {
        eprintln!("CUPS Error: {:?}", cups_last_error_string());
    } else {
        println!("Printer added/modified successfully.");
    }

    ipp_delete(response);
    http_close(http_t);
}