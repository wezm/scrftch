use std::collections::HashMap;

static DISTROS: &'static [&str] = &[
    "unix", "windows", "debian", "ubuntu", "manjaro"
];

lazy_static! {
    pub static ref LOGOS: HashMap <&'static str, &'static str> = {
        let mut logos: HashMap <&'static str, &'static str> = HashMap::new();

        logos.insert("unix", include_str!("logos/unix"));
        logos.insert("debian", include_str!("logos/debian"));
        logos.insert("ubuntu", include_str!("logos/ubuntu"));
        logos.insert("manjaro", include_str!("logos/manjaro"));
        logos.insert("windows", include_str!("logos/windows"));

        /*
        for distro in DISTROS {
            logos.insert(distro, include_str!(format!("logos/{}", distro).as_ref()));
        }
        */

        logos
    };
}
