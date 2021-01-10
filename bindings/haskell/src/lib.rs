use std::ffi::CStr;
use std::os::raw::c_char;
// use std::os::raw::c_float;

use tokenizers::models::bpe::BpeBuilder;
use tokenizers::models::bpe::BPE;
// use tokenizers::pre_tokenizers::byte_level::ByteLevel;
// use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::Tokenizer;

/*
#[no_mangle]
pub extern "C" fn mk_roberta_tokenizer(cvocab: *const c_char, cmerges: *const c_char) -> *mut Tokenizer {
    // make this manually for now
    unsafe {
        let vocab = CStr::from_ptr(cvocab);
        let merges = CStr::from_ptr(cmerges);
        if let (Ok(vocab_file), Ok(merges_file)) = (vocab.to_str(), merges.to_str()) {
            let bpe_builder = BPE::from_file(vocab_file, merges_file);
            let bpe = bpe_builder.with_pre_tokenizer(Some(ByteLevel::default())).build().unwrap();
            return Box::into_raw(Box::new(Tokenizer::new(bpe)));
        } else {
            panic!("Unable to read parameters.");
        }
    }
}

#[no_mangle]
pub extern "C" fn bpe_dropout(ptr: *mut BpeBuilder, dropout: c_float) {
    unsafe {
        let builder = {
            assert!(!ptr.is_null());
            &mut *ptr
        };
        // builder.dropout(dropout);
        // TODO
    }
}
*/

#[no_mangle]
pub extern "C" fn mk_bpe_builder_from_files(
    cvocab: *const c_char,
    cmerges: *const c_char,
) -> *mut BpeBuilder {
    unsafe {
        let vocab = CStr::from_ptr(cvocab);
        let merges = CStr::from_ptr(cmerges);
        if let (Ok(vocab_file), Ok(merges_file)) = (vocab.to_str(), merges.to_str()) {
            Box::into_raw(Box::new(BPE::from_file(vocab_file, merges_file)))
        } else {
            panic!("Unable to read parameters.");
        }
    }
}

#[no_mangle]
pub extern "C" fn mk_tokenizer(cvocab: *const c_char, cmerges: *const c_char) -> *mut Tokenizer {
    unsafe {
        let vocab = CStr::from_ptr(cvocab);
        let merges = CStr::from_ptr(cmerges);
        if let (Ok(vocab_file), Ok(merges_file)) = (vocab.to_str(), merges.to_str()) {
            let bpe_builder = BPE::from_file(vocab_file, merges_file);
            let bpe = bpe_builder.build().unwrap();
            return Box::into_raw(Box::new(Tokenizer::new(bpe)));
        } else {
            panic!("Unable to read parameters.");
        }
    }
}

#[no_mangle]
pub extern "C" fn tokenize(text: *const c_char, ptr: *mut Tokenizer) {
    unsafe {
        let cstring = CStr::from_ptr(text);
        let tokenizer = {
            assert!(!ptr.is_null());
            &mut *ptr
        };
        if let Ok(input) = cstring.to_str() {
            let encoding = tokenizer.encode(input, false).unwrap();
            println!("{:?}", encoding.get_tokens());
        }
    }
}

#[no_mangle]
pub extern "C" fn tokenize_test(x: *const c_char) {
    unsafe {
        let cstring = CStr::from_ptr(x);
        if let Ok(input) = cstring.to_str() {
            let bpe_builder = BPE::from_file("roberta-base-vocab.json", "roberta-base-merges.txt");
            let bpe = bpe_builder.dropout(0.1).build().unwrap();
            let tokenizer = Tokenizer::new(bpe);
            let encoding = tokenizer.encode(input, false).unwrap();
            println!("{:?}", encoding.get_tokens());
        } else {
            panic!("Unable to read parameter.");
        }
    }
}