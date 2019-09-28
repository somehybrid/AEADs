//! AES-128-GCM-SIV tests

#[macro_use]
mod common;

use self::common::TestVector;
use aes_gcm_siv::aead::{generic_array::GenericArray, Aead, NewAead, Payload};
use aes_gcm_siv::Aes128GcmSiv;

/// Test vectors from RFC8452 Appendix C.1: AEAD_AES_128_GCM_SIV
/// <https://tools.ietf.org/html/rfc8452#appendix-C.1>
const TEST_VECTORS: &[TestVector<[u8; 16]>] = &[
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"",
        plaintext: b"",
        ciphertext: b"\xdc\x20\xe2\xd8\x3f\x25\x70\x5b\xb4\x9e\x43\x9e\xca\x56\xde\x25"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"",
        plaintext: b"\x01\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\xb5\xd8\x39\x33\x0a\xc7\xb7\x86\x57\x87\x82\xff\xf6\x01\x3b\x81\x5b\x28\x7c\x22\x49\x3a\x36\x4c"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"",
        plaintext: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x73\x23\xea\x61\xd0\x59\x32\x26\x00\x47\xd9\x42\xa4\x97\x8d\xb3\x57\x39\x1a\x0b\xc4\xfd\xec\x8b\x0d\x10\x66\x39"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"",
        plaintext: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x74\x3f\x7c\x80\x77\xab\x25\xf8\x62\x4e\x2e\x94\x85\x79\xcf\x77\x30\x3a\xaf\x90\xf6\xfe\x21\x19\x9c\x60\x68\x57\x74\x37\xa0\xc4"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"",
        plaintext: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x84\xe0\x7e\x62\xba\x83\xa6\x58\x54\x17\x24\x5d\x7e\xc4\x13\xa9\xfe\x42\x7d\x63\x15\xc0\x9b\x57\xce\x45\xf2\xe3\x93\x6a\x94\x45\x1a\x8e\x45\xdc\xd4\x57\x8c\x66\x7c\xd8\x68\x47\xbf\x61\x55\xff"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"",
        plaintext: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x3f\xd2\x4c\xe1\xf5\xa6\x7b\x75\xbf\x23\x51\xf1\x81\xa4\x75\xc7\xb8\x00\xa5\xb4\xd3\xdc\xf7\x01\x06\xb1\xee\xa8\x2f\xa1\xd6\x4d\xf4\x2b\xf7\x22\x61\x22\xfa\x92\xe1\x7a\x40\xee\xaa\xc1\x20\x1b\x5e\x6e\x31\x1d\xbf\x39\x5d\x35\xb0\xfe\x39\xc2\x71\x43\x88\xf8"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"",
        plaintext: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x24\x33\x66\x8f\x10\x58\x19\x0f\x6d\x43\xe3\x60\xf4\xf3\x5c\xd8\xe4\x75\x12\x7c\xfc\xa7\x02\x8e\xa8\xab\x5c\x20\xf7\xab\x2a\xf0\x25\x16\xa2\xbd\xcb\xc0\x8d\x52\x1b\xe3\x7f\xf2\x8c\x15\x2b\xba\x36\x69\x7f\x25\xb4\xcd\x16\x9c\x65\x90\xd1\xdd\x39\x56\x6d\x3f\x8a\x26\x3d\xd3\x17\xaa\x88\xd5\x6b\xdf\x39\x36\xdb\xa7\x5b\xb8"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01",
        plaintext: b"\x02\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x1e\x6d\xab\xa3\x56\x69\xf4\x27\x3b\x0a\x1a\x25\x60\x96\x9c\xdf\x79\x0d\x99\x75\x9a\xbd\x15\x08"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01",
        plaintext: b"\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x29\x6c\x78\x89\xfd\x99\xf4\x19\x17\xf4\x46\x20\x08\x29\x9c\x51\x02\x74\x5a\xaa\x3a\x0c\x46\x9f\xad\x9e\x07\x5a"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01",
        plaintext: b"\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\xe2\xb0\xc5\xda\x79\xa9\x01\xc1\x74\x5f\x70\x05\x25\xcb\x33\x5b\x8f\x89\x36\xec\x03\x9e\x4e\x4b\xb9\x7e\xbd\x8c\x44\x57\x44\x1f"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01",
        plaintext: b"\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x62\x00\x48\xef\x3c\x1e\x73\xe5\x7e\x02\xbb\x85\x62\xc4\x16\xa3\x19\xe7\x3e\x4c\xaa\xc8\xe9\x6a\x1e\xcb\x29\x33\x14\x5a\x1d\x71\xe6\xaf\x6a\x7f\x87\x28\x7d\xa0\x59\xa7\x16\x84\xed\x34\x98\xe1"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01",
        plaintext: b"\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x50\xc8\x30\x3e\xa9\x39\x25\xd6\x40\x90\xd0\x7b\xd1\x09\xdf\xd9\x51\x5a\x5a\x33\x43\x10\x19\xc1\x7d\x93\x46\x59\x99\xa8\xb0\x05\x32\x01\xd7\x23\x12\x0a\x85\x62\xb8\x38\xcd\xff\x25\xbf\x9d\x1e\x6a\x8c\xc3\x86\x5f\x76\x89\x7c\x2e\x4b\x24\x5c\xf3\x1c\x51\xf2"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01",
        plaintext: b"\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        ciphertext: b"\x2f\x5c\x64\x05\x9d\xb5\x5e\xe0\xfb\x84\x7e\xd5\x13\x00\x37\x46\xac\xa4\xe6\x1c\x71\x1b\x5d\xe2\xe7\xa7\x7f\xfd\x02\xda\x42\xfe\xec\x60\x19\x10\xd3\x46\x7b\xb8\xb3\x6e\xbb\xae\xbc\xe5\xfb\xa3\x0d\x36\xc9\x5f\x48\xa3\xe7\x98\x0f\x0e\x7a\xc2\x99\x33\x2a\x80\xcd\xc4\x6a\xe4\x75\x56\x3d\xe0\x37\x00\x1e\xf8\x4a\xe2\x17\x44"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        plaintext: b"\x02\x00\x00\x00",
        ciphertext: b"\xa8\xfe\x3e\x87\x07\xeb\x1f\x84\xfb\x28\xf8\xcb\x73\xde\x8e\x99\xe2\xf4\x8a\x14"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00",
        plaintext: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00",
        ciphertext: b"\x6b\xb0\xfe\xcf\x5d\xed\x9b\x77\xf9\x02\xc7\xd5\xda\x23\x6a\x43\x91\xdd\x02\x97\x24\xaf\xc9\x80\x5e\x97\x6f\x45\x1e\x6d\x87\xf6\xfe\x10\x65\x14"
    },
    TestVector {
        key: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        nonce: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        aad: b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00",
        plaintext: b"\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00",
        ciphertext: b"\x44\xd0\xaa\xf6\xfb\x2f\x1f\x34\xad\xd5\xe8\x06\x4e\x83\xe1\x2a\x2a\xda\xbf\xf9\xb2\xef\x00\xfb\x47\x92\x0c\xc7\x2a\x0c\x0f\x13\xb9\xfd"
    },
    TestVector {
        key: b"\xe6\x60\x21\xd5\xeb\x8e\x4f\x40\x66\xd4\xad\xb9\xc3\x35\x60\xe4",
        nonce: b"\xf4\x6e\x44\xbb\x3d\xa0\x01\x5c\x94\xf7\x08\x87",
        aad: b"",
        plaintext: b"",
        ciphertext: b"\xa4\x19\x4b\x79\x07\x1b\x01\xa8\x7d\x65\xf7\x06\xe3\x94\x95\x78"
    },
    TestVector {
        key: b"\x36\x86\x42\x00\xe0\xea\xf5\x28\x4d\x88\x4a\x0e\x77\xd3\x16\x46",
        nonce: b"\xba\xe8\xe3\x7f\xc8\x34\x41\xb1\x60\x34\x56\x6b",
        aad: b"\x46\xbb\x91\xc3\xc5",
        plaintext: b"\x7a\x80\x6c",
        ciphertext: b"\xaf\x60\xeb\x71\x1b\xd8\x5b\xc1\xe4\xd3\xe0\xa4\x62\xe0\x74\xee\xa4\x28\xa8"
    },
    TestVector {
        key: b"\xae\xdb\x64\xa6\xc5\x90\xbc\x84\xd1\xa5\xe2\x69\xe4\xb4\x78\x01",
        nonce: b"\xaf\xc0\x57\x7e\x34\x69\x9b\x9e\x67\x1f\xdd\x4f",
        aad: b"\xfc\x88\x0c\x94\xa9\x51\x98\x87\x42\x96",
        plaintext: b"\xbd\xc6\x6f\x14\x65\x45",
        ciphertext: b"\xbb\x93\xa3\xe3\x4d\x3c\xd6\xa9\xc4\x55\x45\xcf\xc1\x1f\x03\xad\x74\x3d\xba\x20\xf9\x66"
    },
    TestVector {
        key: b"\xd5\xcc\x1f\xd1\x61\x32\x0b\x69\x20\xce\x07\x78\x7f\x86\x74\x3b",
        nonce: b"\x27\x5d\x1a\xb3\x2f\x6d\x1f\x04\x34\xd8\x84\x8c",
        aad: b"\x04\x67\x87\xf3\xea\x22\xc1\x27\xaa\xf1\x95\xd1\x89\x47\x28",
        plaintext: b"\x11\x77\x44\x1f\x19\x54\x95\x86\x0f",
        ciphertext: b"\x4f\x37\x28\x1f\x7a\xd1\x29\x49\xd0\x1d\x02\xfd\x0c\xd1\x74\xc8\x4f\xc5\xda\xe2\xf6\x0f\x52\xfd\x2b"
    },
    TestVector {
        key: b"\xb3\xfe\xd1\x47\x3c\x52\x8b\x84\x26\xa5\x82\x99\x59\x29\xa1\x49",
        nonce: b"\x9e\x9a\xd8\x78\x0c\x8d\x63\xd0\xab\x41\x49\xc0",
        aad: b"\xc9\x88\x2e\x53\x86\xfd\x9f\x92\xec\x48\x9c\x8f\xde\x2b\xe2\xcf\x97\xe7\x4e\x93",
        plaintext: b"\x9f\x57\x2c\x61\x4b\x47\x45\x91\x44\x74\xe7\xc7",
        ciphertext: b"\xf5\x46\x73\xc5\xdd\xf7\x10\xc7\x45\x64\x1c\x8b\xc1\xdc\x2f\x87\x1f\xb7\x56\x1d\xa1\x28\x6e\x65\x5e\x24\xb7\xb0"
    },
    TestVector {
        key: b"\x2d\x4e\xd8\x7d\xa4\x41\x02\x95\x2e\xf9\x4b\x02\xb8\x05\x24\x9b",
        nonce: b"\xac\x80\xe6\xf6\x14\x55\xbf\xac\x83\x08\xa2\xd4",
        aad: b"\x29\x50\xa7\x0d\x5a\x1d\xb2\x31\x6f\xd5\x68\x37\x8d\xa1\x07\xb5\x2b\x0d\xa5\x52\x10\xcc\x1c\x1b\x0a",
        plaintext: b"\x0d\x8c\x84\x51\x17\x80\x82\x35\x5c\x9e\x94\x0f\xea\x2f\x58",
        ciphertext: b"\xc9\xff\x54\x5e\x07\xb8\x8a\x01\x5f\x05\xb2\x74\x54\x0a\xa1\x83\xb3\x44\x9b\x9f\x39\x55\x2d\xe9\x9d\xc2\x14\xa1\x19\x0b\x0b"
    },
    TestVector {
        key: b"\xbd\xe3\xb2\xf2\x04\xd1\xe9\xf8\xb0\x6b\xc4\x7f\x97\x45\xb3\xd1",
        nonce: b"\xae\x06\x55\x6f\xb6\xaa\x78\x90\xbe\xbc\x18\xfe",
        aad: b"\x18\x60\xf7\x62\xeb\xfb\xd0\x82\x84\xe4\x21\x70\x2d\xe0\xde\x18\xba\xa9\xc9\x59\x62\x91\xb0\x84\x66\xf3\x7d\xe2\x1c\x7f",
        plaintext: b"\x6b\x3d\xb4\xda\x3d\x57\xaa\x94\x84\x2b\x98\x03\xa9\x6e\x07\xfb\x6d\xe7",
        ciphertext: b"\x62\x98\xb2\x96\xe2\x4e\x8c\xc3\x5d\xce\x0b\xed\x48\x4b\x7f\x30\xd5\x80\x3e\x37\x70\x94\xf0\x47\x09\xf6\x4d\x7b\x98\x53\x10\xa4\xdb\x84"
    },
    TestVector {
        key: b"\xf9\x01\xcf\xe8\xa6\x96\x15\xa9\x3f\xdf\x7a\x98\xca\xd4\x81\x79",
        nonce: b"\x62\x45\x70\x9f\xb1\x88\x53\xf6\x8d\x83\x36\x40",
        aad: b"\x75\x76\xf7\x02\x8e\xc6\xeb\x5e\xa7\xe2\x98\x34\x2a\x94\xd4\xb2\x02\xb3\x70\xef\x97\x68\xec\x65\x61\xc4\xfe\x6b\x7e\x72\x96\xfa\x85\x9c\x21",
        plaintext: b"\xe4\x2a\x3c\x02\xc2\x5b\x64\x86\x9e\x14\x6d\x7b\x23\x39\x87\xbd\xdf\xc2\x40\x87\x1d",
        ciphertext: b"\x39\x1c\xc3\x28\xd4\x84\xa4\xf4\x64\x06\x18\x1b\xcd\x62\xef\xd9\xb3\xee\x19\x7d\x05\x2d\x15\x50\x6c\x84\xa9\xed\xd6\x5e\x13\xe9\xd2\x4a\x2a\x6e\x70"
    },
];

tests!(Aes128GcmSiv, TEST_VECTORS);
