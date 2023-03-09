// @generated
impl serde::Serialize for ClientSideWeightedRoundRobin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enable_oob_load_report.is_some() {
            len += 1;
        }
        if self.oob_reporting_period.is_some() {
            len += 1;
        }
        if self.blackout_period.is_some() {
            len += 1;
        }
        if self.weight_expiration_period.is_some() {
            len += 1;
        }
        if self.weight_update_period.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.client_side_weighted_round_robin.v3.ClientSideWeightedRoundRobin", len)?;
        if let Some(v) = self.enable_oob_load_report.as_ref() {
            struct_ser.serialize_field("enableOobLoadReport", v)?;
        }
        if let Some(v) = self.oob_reporting_period.as_ref() {
            struct_ser.serialize_field("oobReportingPeriod", v)?;
        }
        if let Some(v) = self.blackout_period.as_ref() {
            struct_ser.serialize_field("blackoutPeriod", v)?;
        }
        if let Some(v) = self.weight_expiration_period.as_ref() {
            struct_ser.serialize_field("weightExpirationPeriod", v)?;
        }
        if let Some(v) = self.weight_update_period.as_ref() {
            struct_ser.serialize_field("weightUpdatePeriod", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientSideWeightedRoundRobin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enable_oob_load_report",
            "enableOobLoadReport",
            "oob_reporting_period",
            "oobReportingPeriod",
            "blackout_period",
            "blackoutPeriod",
            "weight_expiration_period",
            "weightExpirationPeriod",
            "weight_update_period",
            "weightUpdatePeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnableOobLoadReport,
            OobReportingPeriod,
            BlackoutPeriod,
            WeightExpirationPeriod,
            WeightUpdatePeriod,
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
                            "enableOobLoadReport" | "enable_oob_load_report" => Ok(GeneratedField::EnableOobLoadReport),
                            "oobReportingPeriod" | "oob_reporting_period" => Ok(GeneratedField::OobReportingPeriod),
                            "blackoutPeriod" | "blackout_period" => Ok(GeneratedField::BlackoutPeriod),
                            "weightExpirationPeriod" | "weight_expiration_period" => Ok(GeneratedField::WeightExpirationPeriod),
                            "weightUpdatePeriod" | "weight_update_period" => Ok(GeneratedField::WeightUpdatePeriod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientSideWeightedRoundRobin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.client_side_weighted_round_robin.v3.ClientSideWeightedRoundRobin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientSideWeightedRoundRobin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enable_oob_load_report__ = None;
                let mut oob_reporting_period__ = None;
                let mut blackout_period__ = None;
                let mut weight_expiration_period__ = None;
                let mut weight_update_period__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EnableOobLoadReport => {
                            if enable_oob_load_report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableOobLoadReport"));
                            }
                            enable_oob_load_report__ = map.next_value()?;
                        }
                        GeneratedField::OobReportingPeriod => {
                            if oob_reporting_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oobReportingPeriod"));
                            }
                            oob_reporting_period__ = map.next_value()?;
                        }
                        GeneratedField::BlackoutPeriod => {
                            if blackout_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blackoutPeriod"));
                            }
                            blackout_period__ = map.next_value()?;
                        }
                        GeneratedField::WeightExpirationPeriod => {
                            if weight_expiration_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weightExpirationPeriod"));
                            }
                            weight_expiration_period__ = map.next_value()?;
                        }
                        GeneratedField::WeightUpdatePeriod => {
                            if weight_update_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weightUpdatePeriod"));
                            }
                            weight_update_period__ = map.next_value()?;
                        }
                    }
                }
                Ok(ClientSideWeightedRoundRobin {
                    enable_oob_load_report: enable_oob_load_report__,
                    oob_reporting_period: oob_reporting_period__,
                    blackout_period: blackout_period__,
                    weight_expiration_period: weight_expiration_period__,
                    weight_update_period: weight_update_period__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.client_side_weighted_round_robin.v3.ClientSideWeightedRoundRobin", FIELDS, GeneratedVisitor)
    }
}
