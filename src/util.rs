//! Module containing various utility functions related to processing Geometry Dash data

/// Performs RobTop's XOR en-/decoding routine on `encoded` using `key`
///
/// Note that although both `encrypted` and `key` are `str`s, the decryption
/// is done directly on the bytes, and the result of each byte-wise XOR
/// operation is casted to `char`, meaning this function only works for
/// ASCII strings.
#[inline]
pub fn cyclic_xor<T>(encoded: &mut [u8], key: &T)
where
    T: AsRef<[u8]> + ?Sized, // ?Sized needed here because we want for example to accept &[u8], where T would be [u8]
{
    // for_each usually specializes better for iterators
    // Also changed into using ^= for simplicity
    encoded.iter_mut().zip(key.as_ref().iter().cycle()).for_each(|(d, k)| *d ^= k);
}

pub(crate) mod default_to_none {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(to_serialize: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Default + Serialize,
    {
        match to_serialize {
            None => T::default().serialize(serializer),
            Some(ref t) => t.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Default + Deserialize<'de> + PartialEq,
    {
        let deserialized = T::deserialize(deserializer)?;

        if deserialized == T::default() {
            Ok(None)
        } else {
            Ok(Some(deserialized))
        }
    }
}
