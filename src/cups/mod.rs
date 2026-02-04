mod c_interop;
mod ipp;
mod ipp_attribute;
mod enums;

use c_interop::{cups_do_request, cups_last_error, cups_last_error_string, cups_server, http_close, http_connect2, ipp_port, HttpT};
use enums::http_encryption::HttpEncryption;
use enums::ipp_operations::IppOp::CupsAddModifyPrinter;
use enums::ipp_status::IppStatus::OkEventsComplete;
use enums::ipp_tag::IPPTag;
use enums::protocol_families::PF;
use url::Url;
use ipp::Ipp;
use enums::ipp_operations::IppOp;
use crate::gui::printer_setup_dialog::PrinterSetupResult;
use crate::smb::SambaCredentials;

#[derive(Debug, Clone)]
pub(crate) struct CupsManager {
    http_t: *mut HttpT,
    pub ppds: Vec<PpdInfo>,
}

#[derive(Debug, Default, Clone)]
pub struct PpdInfo {
    // PPD file name
    pub name: String,
    // Manufacturer and Model combined
    pub make_and_model: String,
    // Manufacturer only
    pub make: String,
    // Model only
    pub product: String,
}

impl CupsManager {
    pub fn new() -> Self {
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

        let http_t = http_t.expect("Failed to connect to CUPS server");

        if http_t.is_null() {
            panic!("Failed to connect to CUPS server at {}:{}", cups_server, ipp_port);
        }

        let mut this = CupsManager { http_t, ppds: Vec::new() };
        this.fetch_ppds();

        this
    }

    pub fn connect_to_printer(&self, creds: SambaCredentials, url: &Url, setup: &PrinterSetupResult, ppd: Option<&PpdInfo>) {
        let request = Ipp::new(CupsAddModifyPrinter);
        let mut smb_printer_uri = url.clone();

        smb_printer_uri
            .set_username(&*creds.username)
            .expect("Unable to set username");

        smb_printer_uri
            .set_password(Option::from(&*creds.password))
            .expect("Unable to set password");

        // Create printer URI and escape printer name
        let mut printer_ipp_uri = Url::parse("ipp://localhost/printers/").unwrap();
        printer_ipp_uri = printer_ipp_uri.join(&setup.printer_name).expect("Unable to create printer URI");

        request.add_string(IPPTag::Operation, IPPTag::Uri,
                           Option::from("printer-uri"), None, &printer_ipp_uri.to_string());

        request.add_string(IPPTag::Printer, IPPTag::Uri,
                       Option::from("device-uri"), None,
                       &smb_printer_uri.to_string());

        request.add_string(IPPTag::Printer, IPPTag::Name,
                       Option::from("printer-location"), None,
                       &setup.location);


        if let Some(ppd) = ppd {
            request.add_string(IPPTag::Printer, IPPTag::Name,
                           Option::from("ppd-name"), None,
                           &ppd.name);
        }


        let response = cups_do_request(self.http_t, request.into_raw(), "/admin/");

        let _response = response
            .and_then(Ipp::from_raw);

        if cups_last_error() > OkEventsComplete {
            eprintln!("CUPS Error: {:?}", cups_last_error_string());
        } else {
            println!("Printer added/modified successfully.");
        }
    }

    fn fetch_ppds(&mut self) -> bool {
        let request = Ipp::new(IppOp::CupsGetPpds);

        request.add_string(IPPTag::Operation, IPPTag::Uri, Option::from("printer-uri"),
                       None, "ipp://localhost/",
        );

        let response = cups_do_request(self.http_t, request.into_raw(), "/");

        if response.is_none() {
            eprintln!("CUPS request failed");
            return false;
        }

        let mut response = match response.and_then(Ipp::from_raw) {
            Some(res) => res,
            None => {
                eprintln!("CUPS request failed");
                return false;
            }
        };

        self.parse_response(&mut response);

        true
    }

    fn parse_response(&mut self, response: &mut Ipp) {
        let mut current = PpdInfo::default();

        let mut attr = response.get_first_attribute();
        while let Some(attr_ptr) = attr {
            let attr_ref = unsafe { attr_ptr.as_mut() };
            if let Some(attr_ref) = attr_ref {
                if attr_ref.get_group_tag() == IPPTag::Printer {
                    let name_ptr = attr_ref.get_name();
                    if let Some(attr_name) = name_ptr {

                        match attr_name.as_str() {
                            "ppd-name" => { // PPD file name
                                current = PpdInfo::default();
                                current.name = attr_ref.get_string().unwrap_or_default();
                            }
                            "ppd-make-and-model" => { // Manufacturer and Model combined
                                current.make_and_model = attr_ref.get_string().unwrap_or_default();
                            }
                            "ppd-make" => { // Manufacturer only
                                current.make = attr_ref.get_string().unwrap_or_default();
                            }
                            "ppd-product" => { // Model only
                                current.product = attr_ref.get_string().unwrap_or_default();

                                // "ppd-product" marks also the end of one PPD entry
                                self.ppds.push(current.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }

            attr = response.get_next_attribute();
        }
    }
}

impl Drop for CupsManager {
    fn drop(&mut self) {
        http_close(self.http_t);
    }
}