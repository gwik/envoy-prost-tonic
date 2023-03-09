// @generated
impl serde::Serialize for ProfileActionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.profile_duration.is_some() {
            len += 1;
        }
        if !self.profile_path.is_empty() {
            len += 1;
        }
        if self.max_profiles != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.watchdog.profile_action.v3.ProfileActionConfig", len)?;
        if let Some(v) = self.profile_duration.as_ref() {
            struct_ser.serialize_field("profileDuration", v)?;
        }
        if !self.profile_path.is_empty() {
            struct_ser.serialize_field("profilePath", &self.profile_path)?;
        }
        if self.max_profiles != 0 {
            struct_ser.serialize_field("maxProfiles", ToString::to_string(&self.max_profiles).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProfileActionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "profile_duration",
            "profileDuration",
            "profile_path",
            "profilePath",
            "max_profiles",
            "maxProfiles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProfileDuration,
            ProfilePath,
            MaxProfiles,
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
                            "profileDuration" | "profile_duration" => Ok(GeneratedField::ProfileDuration),
                            "profilePath" | "profile_path" => Ok(GeneratedField::ProfilePath),
                            "maxProfiles" | "max_profiles" => Ok(GeneratedField::MaxProfiles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProfileActionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.watchdog.profile_action.v3.ProfileActionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProfileActionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut profile_duration__ = None;
                let mut profile_path__ = None;
                let mut max_profiles__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProfileDuration => {
                            if profile_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profileDuration"));
                            }
                            profile_duration__ = map.next_value()?;
                        }
                        GeneratedField::ProfilePath => {
                            if profile_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profilePath"));
                            }
                            profile_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxProfiles => {
                            if max_profiles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxProfiles"));
                            }
                            max_profiles__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProfileActionConfig {
                    profile_duration: profile_duration__,
                    profile_path: profile_path__.unwrap_or_default(),
                    max_profiles: max_profiles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.watchdog.profile_action.v3.ProfileActionConfig", FIELDS, GeneratedVisitor)
    }
}
