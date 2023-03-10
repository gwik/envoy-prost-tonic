// @generated
impl serde::Serialize for Hyperscan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.regexes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.matching.input_matchers.hyperscan.v3alpha.Hyperscan", len)?;
        if !self.regexes.is_empty() {
            struct_ser.serialize_field("regexes", &self.regexes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Hyperscan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "regexes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Regexes,
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
                            "regexes" => Ok(GeneratedField::Regexes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Hyperscan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.matching.input_matchers.hyperscan.v3alpha.Hyperscan")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Hyperscan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut regexes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Regexes => {
                            if regexes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexes"));
                            }
                            regexes__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Hyperscan {
                    regexes: regexes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.matching.input_matchers.hyperscan.v3alpha.Hyperscan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for hyperscan::Regex {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.regex.is_empty() {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        if self.caseless {
            len += 1;
        }
        if self.dot_all {
            len += 1;
        }
        if self.multiline {
            len += 1;
        }
        if self.allow_empty {
            len += 1;
        }
        if self.utf8 {
            len += 1;
        }
        if self.ucp {
            len += 1;
        }
        if self.combination {
            len += 1;
        }
        if self.quiet {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.matching.input_matchers.hyperscan.v3alpha.Hyperscan.Regex", len)?;
        if !self.regex.is_empty() {
            struct_ser.serialize_field("regex", &self.regex)?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.caseless {
            struct_ser.serialize_field("caseless", &self.caseless)?;
        }
        if self.dot_all {
            struct_ser.serialize_field("dotAll", &self.dot_all)?;
        }
        if self.multiline {
            struct_ser.serialize_field("multiline", &self.multiline)?;
        }
        if self.allow_empty {
            struct_ser.serialize_field("allowEmpty", &self.allow_empty)?;
        }
        if self.utf8 {
            struct_ser.serialize_field("utf8", &self.utf8)?;
        }
        if self.ucp {
            struct_ser.serialize_field("ucp", &self.ucp)?;
        }
        if self.combination {
            struct_ser.serialize_field("combination", &self.combination)?;
        }
        if self.quiet {
            struct_ser.serialize_field("quiet", &self.quiet)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for hyperscan::Regex {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "regex",
            "id",
            "caseless",
            "dot_all",
            "dotAll",
            "multiline",
            "allow_empty",
            "allowEmpty",
            "utf8",
            "ucp",
            "combination",
            "quiet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Regex,
            Id,
            Caseless,
            DotAll,
            Multiline,
            AllowEmpty,
            Utf8,
            Ucp,
            Combination,
            Quiet,
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
                            "regex" => Ok(GeneratedField::Regex),
                            "id" => Ok(GeneratedField::Id),
                            "caseless" => Ok(GeneratedField::Caseless),
                            "dotAll" | "dot_all" => Ok(GeneratedField::DotAll),
                            "multiline" => Ok(GeneratedField::Multiline),
                            "allowEmpty" | "allow_empty" => Ok(GeneratedField::AllowEmpty),
                            "utf8" => Ok(GeneratedField::Utf8),
                            "ucp" => Ok(GeneratedField::Ucp),
                            "combination" => Ok(GeneratedField::Combination),
                            "quiet" => Ok(GeneratedField::Quiet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = hyperscan::Regex;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.matching.input_matchers.hyperscan.v3alpha.Hyperscan.Regex")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<hyperscan::Regex, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut regex__ = None;
                let mut id__ = None;
                let mut caseless__ = None;
                let mut dot_all__ = None;
                let mut multiline__ = None;
                let mut allow_empty__ = None;
                let mut utf8__ = None;
                let mut ucp__ = None;
                let mut combination__ = None;
                let mut quiet__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Regex => {
                            if regex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            regex__ = Some(map.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Caseless => {
                            if caseless__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caseless"));
                            }
                            caseless__ = Some(map.next_value()?);
                        }
                        GeneratedField::DotAll => {
                            if dot_all__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dotAll"));
                            }
                            dot_all__ = Some(map.next_value()?);
                        }
                        GeneratedField::Multiline => {
                            if multiline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiline"));
                            }
                            multiline__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowEmpty => {
                            if allow_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowEmpty"));
                            }
                            allow_empty__ = Some(map.next_value()?);
                        }
                        GeneratedField::Utf8 => {
                            if utf8__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utf8"));
                            }
                            utf8__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ucp => {
                            if ucp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ucp"));
                            }
                            ucp__ = Some(map.next_value()?);
                        }
                        GeneratedField::Combination => {
                            if combination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("combination"));
                            }
                            combination__ = Some(map.next_value()?);
                        }
                        GeneratedField::Quiet => {
                            if quiet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quiet"));
                            }
                            quiet__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(hyperscan::Regex {
                    regex: regex__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    caseless: caseless__.unwrap_or_default(),
                    dot_all: dot_all__.unwrap_or_default(),
                    multiline: multiline__.unwrap_or_default(),
                    allow_empty: allow_empty__.unwrap_or_default(),
                    utf8: utf8__.unwrap_or_default(),
                    ucp: ucp__.unwrap_or_default(),
                    combination: combination__.unwrap_or_default(),
                    quiet: quiet__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.matching.input_matchers.hyperscan.v3alpha.Hyperscan.Regex", FIELDS, GeneratedVisitor)
    }
}
