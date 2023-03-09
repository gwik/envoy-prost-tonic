// @generated
impl serde::Serialize for TlsInspector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enable_ja3_fingerprinting.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.listener.tls_inspector.v3.TlsInspector", len)?;
        if let Some(v) = self.enable_ja3_fingerprinting.as_ref() {
            struct_ser.serialize_field("enableJa3Fingerprinting", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsInspector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enable_ja3_fingerprinting",
            "enableJa3Fingerprinting",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnableJa3Fingerprinting,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "enableJa3Fingerprinting" | "enable_ja3_fingerprinting" => Ok(GeneratedField::EnableJa3Fingerprinting),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsInspector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.listener.tls_inspector.v3.TlsInspector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TlsInspector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enable_ja3_fingerprinting__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EnableJa3Fingerprinting => {
                            if enable_ja3_fingerprinting__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableJa3Fingerprinting"));
                            }
                            enable_ja3_fingerprinting__ = map.next_value()?;
                        }
                    }
                }
                Ok(TlsInspector {
                    enable_ja3_fingerprinting: enable_ja3_fingerprinting__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.listener.tls_inspector.v3.TlsInspector", FIELDS, GeneratedVisitor)
    }
}
