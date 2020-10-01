use docblock::SourceFile;
use sha2::{Digest, Sha256};

// Sign source file by adding
//      @generated
//      @signed_source abc14121213acdabe131313
// directives to its docblock. where @signed_source has a value of sha256 hash
// of the rest of the file (without docblock)
//
// We sign files in order to guarantee tha they're not modified manually
// by adding a test that will look for all @signed_source directives in
// the file, hashing the rest of the file and comparing them.
// If the hash is @signed_source directive doesn't match produced hash, that
// means the file was manually modified and source needs to be regenerated.
pub fn sign_source(source: &str) -> String {
    let mut source_file = SourceFile::from_source(source);
    let hash = hex::encode(Sha256::digest(source_file.rest.as_bytes()));

    source_file.set_directive("generated", None);
    source_file.set_directive("signed_source", Some(&hash));
    source_file.to_source()
}

#[cfg(test)]
mod tests {
    use super::*;
    use k9::*;

    #[test]
    fn test_signature() {
        let source = r#"
let a = "hello world";
console.log(1 + a);
        "#;

        let signed = sign_source(source);

        assert_matches_inline_snapshot!(
            signed,
            "/*
 * @generated
 * @signed_source 9c5583c9e6a1018a3ea5e32943816cd0dadd967f714aba23e31d67fd388a417d
 */


let a = \"hello world\";
console.log(1 + a);
        "
        );
    }
}
