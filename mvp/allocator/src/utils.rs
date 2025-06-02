use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

/// Format datacap size (in bytes, as string) into human-readable units like GiB, TiB, etc.
pub fn format_datacap_size_str(bytes_str: &str) -> String {
    let bytes = bytes_str.parse::<u64>().unwrap_or(0);
    format_datacap_size(bytes)
}

pub fn format_datacap_size(bytes: u64) -> String {

    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;
    const TIB: u64 = GIB * 1024;
    const PIB: u64 = TIB * 1024;

    let (value, unit) = if bytes >= PIB {
        (bytes / PIB, "PiB")
    } else if bytes >= TIB {
        (bytes / TIB, "TiB")
    } else if bytes >= GIB {
        (bytes / GIB, "GiB")
    } else if bytes >= MIB {
        (bytes / MIB, "MiB")
    } else if bytes >= KIB {
        (bytes / KIB, "KiB")
    } else {
        (bytes, "B")
    };

    format!("{:.3} {}", value, unit)
}

pub fn fil_to_atto_string(fil: f64) -> String {
    let fil_dec = Decimal::from_f64(fil).unwrap_or(dec!(0));
    let atto_dec = fil_dec * dec!(1_000_000_000_000_000_000);
    atto_dec.round().to_string()
}

/// Converts attoFIL (string) to FIL (float). //TODO in utils
pub fn wei_to_fil(wei: &str) -> f64 {
    wei.parse::<f64>().unwrap_or(0.0) / 1e18
}

