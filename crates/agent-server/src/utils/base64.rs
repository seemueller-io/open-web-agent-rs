use base64::Engine;
use base64::engine::GeneralPurpose;
use base64::engine::general_purpose::STANDARD;
use base64::engine::general_purpose::STANDARD_NO_PAD;

pub struct Base64Encoder {
    payload_engine: GeneralPurpose,
    signature_engine: GeneralPurpose,
    public_key_engine: GeneralPurpose,
    secret_key_engine: GeneralPurpose,
}

impl Base64Encoder {
    pub(crate) fn b64_encode(&self, p0: &[u8]) -> String {
        self.payload_engine.encode(p0)
    }
    pub(crate) fn b64_decode(&self, p0: String) -> Result<Vec<u8>, base64::DecodeError> {
        self.payload_engine.decode(p0)
    }
}

pub const B64_ENCODER: &Base64Encoder = &Base64Encoder::new();

impl Base64Encoder {
    pub const fn new() -> Self { // Made new() a const fn
        Base64Encoder {
            payload_engine: STANDARD,
            signature_engine: STANDARD,
            public_key_engine: STANDARD,
            secret_key_engine: STANDARD,
        }
    }

    pub fn b64_encode_payload<T: AsRef<[u8]>>(&self, input: T) -> String { // Added trait bound
        self.payload_engine.encode(input)
    }

    pub fn b64_decode_payload<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, base64::DecodeError> { // Added trait bound
        self.payload_engine.decode(input)
    }

    pub fn b64_decode_signature<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, base64::DecodeError> { // Added trait bound
        self.signature_engine.decode(input)
    }

    pub fn b64_encode_signature<T: AsRef<[u8]>>(&self, input: T) -> String { // Added trait bound
        self.signature_engine.encode(input)
    }

    pub fn b64_encode_public_key<T: AsRef<[u8]>>(&self, input: T) -> String { // Added trait bound
        self.public_key_engine.encode(input)
    }

    pub fn b64_decode_public_key<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, base64::DecodeError> { // Added trait bound
        self.public_key_engine.decode(input)
    }

    pub fn b64_encode_secret_key<T: AsRef<[u8]>>(&self, input: T) -> String { // Added trait bound
        self.secret_key_engine.encode(input)
    }

    pub fn b64_decode_secret_key<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, base64::DecodeError> { // Added trait bound
        self.secret_key_engine.decode(input)
    }
}
