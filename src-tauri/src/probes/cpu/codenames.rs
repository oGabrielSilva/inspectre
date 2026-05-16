pub fn lookup(vendor: &str, family: u8, model: u8) -> Option<&'static str> {
    match vendor {
        "GenuineIntel" => intel_codename(family, model),
        "AuthenticAMD" => amd_codename(family, model),
        _ => None,
    }
}

fn intel_codename(family: u8, model: u8) -> Option<&'static str> {
    if family != 0x06 {
        return None;
    }
    match model {
        0x3C | 0x45 | 0x46 => Some("Haswell"),
        0x3D | 0x47 | 0x4F | 0x56 => Some("Broadwell"),
        0x4E | 0x5E | 0x55 => Some("Skylake"),
        0x8E | 0x9E => Some("Kaby Lake / Coffee Lake"),
        0xA5 | 0xA6 => Some("Comet Lake"),
        0x66 => Some("Cannon Lake"),
        0x7D | 0x7E => Some("Ice Lake"),
        0x8C | 0x8D => Some("Tiger Lake"),
        0xA7 | 0xA8 => Some("Rocket Lake"),
        0x97 | 0x9A | 0xBE => Some("Alder Lake"),
        0xB7 | 0xBA | 0xBF => Some("Raptor Lake"),
        0xAA | 0xAC => Some("Meteor Lake"),
        0xC6 => Some("Arrow Lake"),
        0xBD => Some("Lunar Lake"),
        _ => None,
    }
}

fn amd_codename(family: u8, model: u8) -> Option<&'static str> {
    match (family, model) {
        (0x17, 0x01) | (0x17, 0x11) => Some("Zen / Summit Ridge"),
        (0x17, 0x08) | (0x17, 0x18) => Some("Zen+ / Pinnacle Ridge"),
        (0x17, 0x31) | (0x17, 0x60) | (0x17, 0x71) => Some("Zen 2 / Matisse"),
        (0x19, 0x21) | (0x19, 0x50) => Some("Zen 3 / Vermeer"),
        (0x19, 0x40) | (0x19, 0x44) => Some("Zen 3+ / Rembrandt"),
        (0x19, 0x61) | (0x19, 0x70) => Some("Zen 4 / Raphael"),
        (0x1A, 0x44) | (0x1A, 0x24) => Some("Zen 5 / Granite Ridge"),
        _ => None,
    }
}
