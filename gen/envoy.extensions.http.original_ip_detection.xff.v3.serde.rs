// @generated
impl serde::Serialize for XffConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.xff_num_trusted_hops != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.original_ip_detection.xff.v3.XffConfig", len)?;
        if self.xff_num_trusted_hops != 0 {
            struct_ser.serialize_field("xffNumTrustedHops", &self.xff_num_trusted_hops)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for XffConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "xff_num_trusted_hops",
            "xffNumTrustedHops",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            XffNumTrustedHops,
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
                            "xffNumTrustedHops" | "xff_num_trusted_hops" => Ok(GeneratedField::XffNumTrustedHops),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = XffConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.original_ip_detection.xff.v3.XffConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<XffConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut xff_num_trusted_hops__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::XffNumTrustedHops => {
                            if xff_num_trusted_hops__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xffNumTrustedHops"));
                            }
                            xff_num_trusted_hops__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(XffConfig {
                    xff_num_trusted_hops: xff_num_trusted_hops__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.original_ip_detection.xff.v3.XffConfig", FIELDS, GeneratedVisitor)
    }
}
