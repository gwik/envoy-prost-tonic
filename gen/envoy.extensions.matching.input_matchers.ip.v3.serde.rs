// @generated
impl serde::Serialize for Ip {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cidr_ranges.is_empty() {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.matching.input_matchers.ip.v3.Ip", len)?;
        if !self.cidr_ranges.is_empty() {
            struct_ser.serialize_field("cidrRanges", &self.cidr_ranges)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ip {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cidr_ranges",
            "cidrRanges",
            "stat_prefix",
            "statPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CidrRanges,
            StatPrefix,
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
                            "cidrRanges" | "cidr_ranges" => Ok(GeneratedField::CidrRanges),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ip;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.matching.input_matchers.ip.v3.Ip")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Ip, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cidr_ranges__ = None;
                let mut stat_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CidrRanges => {
                            if cidr_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cidrRanges"));
                            }
                            cidr_ranges__ = Some(map.next_value()?);
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Ip {
                    cidr_ranges: cidr_ranges__.unwrap_or_default(),
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.matching.input_matchers.ip.v3.Ip", FIELDS, GeneratedVisitor)
    }
}
