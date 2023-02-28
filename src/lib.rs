mod math {
    mod math_js {
        #[link(wasm_import_module = "Math")]
        extern "C" {
            pub fn random() -> f64;
        }
    }

    pub fn random() -> f64 {
        unsafe { math_js::random() }
    }
}

const _: () = {
    #[link_section = "surmsection"]
    static SECTION_CONTENT: [u8; 11] = *b"hello world";
};

#[export_name = "add"]
pub fn add(left: f64, right: f64) -> f64 {
    left + right + unsafe { math::random() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
