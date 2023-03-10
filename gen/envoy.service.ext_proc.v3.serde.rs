// @generated
impl serde::Serialize for BodyMutation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mutation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.BodyMutation", len)?;
        if let Some(v) = self.mutation.as_ref() {
            match v {
                body_mutation::Mutation::Body(v) => {
                    struct_ser.serialize_field("body", pbjson::private::base64::encode(&v).as_str())?;
                }
                body_mutation::Mutation::ClearBody(v) => {
                    struct_ser.serialize_field("clearBody", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BodyMutation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "body",
            "clear_body",
            "clearBody",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Body,
            ClearBody,
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
                            "body" => Ok(GeneratedField::Body),
                            "clearBody" | "clear_body" => Ok(GeneratedField::ClearBody),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BodyMutation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.BodyMutation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BodyMutation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mutation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Body => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            mutation__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| body_mutation::Mutation::Body(x.0));
                        }
                        GeneratedField::ClearBody => {
                            if mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clearBody"));
                            }
                            mutation__ = map.next_value::<::std::option::Option<_>>()?.map(body_mutation::Mutation::ClearBody);
                        }
                    }
                }
                Ok(BodyMutation {
                    mutation: mutation__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.BodyMutation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BodyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.BodyResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BodyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Response,
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
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BodyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.BodyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BodyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map.next_value()?;
                        }
                    }
                }
                Ok(BodyResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.BodyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommonResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.header_mutation.is_some() {
            len += 1;
        }
        if self.body_mutation.is_some() {
            len += 1;
        }
        if self.trailers.is_some() {
            len += 1;
        }
        if self.clear_route_cache {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.CommonResponse", len)?;
        if self.status != 0 {
            let v = common_response::ResponseStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.header_mutation.as_ref() {
            struct_ser.serialize_field("headerMutation", v)?;
        }
        if let Some(v) = self.body_mutation.as_ref() {
            struct_ser.serialize_field("bodyMutation", v)?;
        }
        if let Some(v) = self.trailers.as_ref() {
            struct_ser.serialize_field("trailers", v)?;
        }
        if self.clear_route_cache {
            struct_ser.serialize_field("clearRouteCache", &self.clear_route_cache)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommonResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "header_mutation",
            "headerMutation",
            "body_mutation",
            "bodyMutation",
            "trailers",
            "clear_route_cache",
            "clearRouteCache",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            HeaderMutation,
            BodyMutation,
            Trailers,
            ClearRouteCache,
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
                            "status" => Ok(GeneratedField::Status),
                            "headerMutation" | "header_mutation" => Ok(GeneratedField::HeaderMutation),
                            "bodyMutation" | "body_mutation" => Ok(GeneratedField::BodyMutation),
                            "trailers" => Ok(GeneratedField::Trailers),
                            "clearRouteCache" | "clear_route_cache" => Ok(GeneratedField::ClearRouteCache),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommonResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.CommonResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommonResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut header_mutation__ = None;
                let mut body_mutation__ = None;
                let mut trailers__ = None;
                let mut clear_route_cache__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<common_response::ResponseStatus>()? as i32);
                        }
                        GeneratedField::HeaderMutation => {
                            if header_mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerMutation"));
                            }
                            header_mutation__ = map.next_value()?;
                        }
                        GeneratedField::BodyMutation => {
                            if body_mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodyMutation"));
                            }
                            body_mutation__ = map.next_value()?;
                        }
                        GeneratedField::Trailers => {
                            if trailers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trailers"));
                            }
                            trailers__ = map.next_value()?;
                        }
                        GeneratedField::ClearRouteCache => {
                            if clear_route_cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clearRouteCache"));
                            }
                            clear_route_cache__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommonResponse {
                    status: status__.unwrap_or_default(),
                    header_mutation: header_mutation__,
                    body_mutation: body_mutation__,
                    trailers: trailers__,
                    clear_route_cache: clear_route_cache__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.CommonResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for common_response::ResponseStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Continue => "CONTINUE",
            Self::ContinueAndReplace => "CONTINUE_AND_REPLACE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for common_response::ResponseStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTINUE",
            "CONTINUE_AND_REPLACE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = common_response::ResponseStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(common_response::ResponseStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(common_response::ResponseStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CONTINUE" => Ok(common_response::ResponseStatus::Continue),
                    "CONTINUE_AND_REPLACE" => Ok(common_response::ResponseStatus::ContinueAndReplace),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GrpcStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.GrpcStatus", len)?;
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrpcStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
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
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrpcStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.GrpcStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GrpcStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GrpcStatus {
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.GrpcStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderMutation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.set_headers.is_empty() {
            len += 1;
        }
        if !self.remove_headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.HeaderMutation", len)?;
        if !self.set_headers.is_empty() {
            struct_ser.serialize_field("setHeaders", &self.set_headers)?;
        }
        if !self.remove_headers.is_empty() {
            struct_ser.serialize_field("removeHeaders", &self.remove_headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderMutation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "set_headers",
            "setHeaders",
            "remove_headers",
            "removeHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SetHeaders,
            RemoveHeaders,
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
                            "setHeaders" | "set_headers" => Ok(GeneratedField::SetHeaders),
                            "removeHeaders" | "remove_headers" => Ok(GeneratedField::RemoveHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderMutation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.HeaderMutation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderMutation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut set_headers__ = None;
                let mut remove_headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SetHeaders => {
                            if set_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setHeaders"));
                            }
                            set_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemoveHeaders => {
                            if remove_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeHeaders"));
                            }
                            remove_headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HeaderMutation {
                    set_headers: set_headers__.unwrap_or_default(),
                    remove_headers: remove_headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.HeaderMutation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeadersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.HeadersResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeadersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Response,
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
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeadersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.HeadersResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeadersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map.next_value()?;
                        }
                    }
                }
                Ok(HeadersResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.HeadersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpBody {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.body.is_empty() {
            len += 1;
        }
        if self.end_of_stream {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.HttpBody", len)?;
        if !self.body.is_empty() {
            struct_ser.serialize_field("body", pbjson::private::base64::encode(&self.body).as_str())?;
        }
        if self.end_of_stream {
            struct_ser.serialize_field("endOfStream", &self.end_of_stream)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpBody {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "body",
            "end_of_stream",
            "endOfStream",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Body,
            EndOfStream,
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
                            "body" => Ok(GeneratedField::Body),
                            "endOfStream" | "end_of_stream" => Ok(GeneratedField::EndOfStream),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpBody;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.HttpBody")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpBody, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut body__ = None;
                let mut end_of_stream__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EndOfStream => {
                            if end_of_stream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endOfStream"));
                            }
                            end_of_stream__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HttpBody {
                    body: body__.unwrap_or_default(),
                    end_of_stream: end_of_stream__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.HttpBody", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpHeaders {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.headers.is_some() {
            len += 1;
        }
        if !self.attributes.is_empty() {
            len += 1;
        }
        if self.end_of_stream {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.HttpHeaders", len)?;
        if let Some(v) = self.headers.as_ref() {
            struct_ser.serialize_field("headers", v)?;
        }
        if !self.attributes.is_empty() {
            struct_ser.serialize_field("attributes", &self.attributes)?;
        }
        if self.end_of_stream {
            struct_ser.serialize_field("endOfStream", &self.end_of_stream)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpHeaders {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "headers",
            "attributes",
            "end_of_stream",
            "endOfStream",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Headers,
            Attributes,
            EndOfStream,
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
                            "headers" => Ok(GeneratedField::Headers),
                            "attributes" => Ok(GeneratedField::Attributes),
                            "endOfStream" | "end_of_stream" => Ok(GeneratedField::EndOfStream),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpHeaders;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.HttpHeaders")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpHeaders, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut headers__ = None;
                let mut attributes__ = None;
                let mut end_of_stream__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::EndOfStream => {
                            if end_of_stream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endOfStream"));
                            }
                            end_of_stream__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HttpHeaders {
                    headers: headers__,
                    attributes: attributes__.unwrap_or_default(),
                    end_of_stream: end_of_stream__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.HttpHeaders", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpTrailers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trailers.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.HttpTrailers", len)?;
        if let Some(v) = self.trailers.as_ref() {
            struct_ser.serialize_field("trailers", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpTrailers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trailers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Trailers,
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
                            "trailers" => Ok(GeneratedField::Trailers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpTrailers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.HttpTrailers")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpTrailers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trailers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Trailers => {
                            if trailers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trailers"));
                            }
                            trailers__ = map.next_value()?;
                        }
                    }
                }
                Ok(HttpTrailers {
                    trailers: trailers__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.HttpTrailers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImmediateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        if self.headers.is_some() {
            len += 1;
        }
        if !self.body.is_empty() {
            len += 1;
        }
        if self.grpc_status.is_some() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.ImmediateResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.headers.as_ref() {
            struct_ser.serialize_field("headers", v)?;
        }
        if !self.body.is_empty() {
            struct_ser.serialize_field("body", &self.body)?;
        }
        if let Some(v) = self.grpc_status.as_ref() {
            struct_ser.serialize_field("grpcStatus", v)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImmediateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "headers",
            "body",
            "grpc_status",
            "grpcStatus",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Headers,
            Body,
            GrpcStatus,
            Details,
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
                            "status" => Ok(GeneratedField::Status),
                            "headers" => Ok(GeneratedField::Headers),
                            "body" => Ok(GeneratedField::Body),
                            "grpcStatus" | "grpc_status" => Ok(GeneratedField::GrpcStatus),
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImmediateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.ImmediateResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ImmediateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut headers__ = None;
                let mut body__ = None;
                let mut grpc_status__ = None;
                let mut details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = map.next_value()?;
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = Some(map.next_value()?);
                        }
                        GeneratedField::GrpcStatus => {
                            if grpc_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcStatus"));
                            }
                            grpc_status__ = map.next_value()?;
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ImmediateResponse {
                    status: status__,
                    headers: headers__,
                    body: body__.unwrap_or_default(),
                    grpc_status: grpc_status__,
                    details: details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.ImmediateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProcessingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.async_mode {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.ProcessingRequest", len)?;
        if self.async_mode {
            struct_ser.serialize_field("asyncMode", &self.async_mode)?;
        }
        if let Some(v) = self.request.as_ref() {
            match v {
                processing_request::Request::RequestHeaders(v) => {
                    struct_ser.serialize_field("requestHeaders", v)?;
                }
                processing_request::Request::ResponseHeaders(v) => {
                    struct_ser.serialize_field("responseHeaders", v)?;
                }
                processing_request::Request::RequestBody(v) => {
                    struct_ser.serialize_field("requestBody", v)?;
                }
                processing_request::Request::ResponseBody(v) => {
                    struct_ser.serialize_field("responseBody", v)?;
                }
                processing_request::Request::RequestTrailers(v) => {
                    struct_ser.serialize_field("requestTrailers", v)?;
                }
                processing_request::Request::ResponseTrailers(v) => {
                    struct_ser.serialize_field("responseTrailers", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProcessingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "async_mode",
            "asyncMode",
            "request_headers",
            "requestHeaders",
            "response_headers",
            "responseHeaders",
            "request_body",
            "requestBody",
            "response_body",
            "responseBody",
            "request_trailers",
            "requestTrailers",
            "response_trailers",
            "responseTrailers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AsyncMode,
            RequestHeaders,
            ResponseHeaders,
            RequestBody,
            ResponseBody,
            RequestTrailers,
            ResponseTrailers,
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
                            "asyncMode" | "async_mode" => Ok(GeneratedField::AsyncMode),
                            "requestHeaders" | "request_headers" => Ok(GeneratedField::RequestHeaders),
                            "responseHeaders" | "response_headers" => Ok(GeneratedField::ResponseHeaders),
                            "requestBody" | "request_body" => Ok(GeneratedField::RequestBody),
                            "responseBody" | "response_body" => Ok(GeneratedField::ResponseBody),
                            "requestTrailers" | "request_trailers" => Ok(GeneratedField::RequestTrailers),
                            "responseTrailers" | "response_trailers" => Ok(GeneratedField::ResponseTrailers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProcessingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.ProcessingRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProcessingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut async_mode__ = None;
                let mut request__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AsyncMode => {
                            if async_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asyncMode"));
                            }
                            async_mode__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeaders => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaders"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(processing_request::Request::RequestHeaders)
;
                        }
                        GeneratedField::ResponseHeaders => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeaders"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(processing_request::Request::ResponseHeaders)
;
                        }
                        GeneratedField::RequestBody => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestBody"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(processing_request::Request::RequestBody)
;
                        }
                        GeneratedField::ResponseBody => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseBody"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(processing_request::Request::ResponseBody)
;
                        }
                        GeneratedField::RequestTrailers => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTrailers"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(processing_request::Request::RequestTrailers)
;
                        }
                        GeneratedField::ResponseTrailers => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTrailers"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(processing_request::Request::ResponseTrailers)
;
                        }
                    }
                }
                Ok(ProcessingRequest {
                    async_mode: async_mode__.unwrap_or_default(),
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.ProcessingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProcessingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dynamic_metadata.is_some() {
            len += 1;
        }
        if self.mode_override.is_some() {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.ProcessingResponse", len)?;
        if let Some(v) = self.dynamic_metadata.as_ref() {
            struct_ser.serialize_field("dynamicMetadata", v)?;
        }
        if let Some(v) = self.mode_override.as_ref() {
            struct_ser.serialize_field("modeOverride", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            match v {
                processing_response::Response::RequestHeaders(v) => {
                    struct_ser.serialize_field("requestHeaders", v)?;
                }
                processing_response::Response::ResponseHeaders(v) => {
                    struct_ser.serialize_field("responseHeaders", v)?;
                }
                processing_response::Response::RequestBody(v) => {
                    struct_ser.serialize_field("requestBody", v)?;
                }
                processing_response::Response::ResponseBody(v) => {
                    struct_ser.serialize_field("responseBody", v)?;
                }
                processing_response::Response::RequestTrailers(v) => {
                    struct_ser.serialize_field("requestTrailers", v)?;
                }
                processing_response::Response::ResponseTrailers(v) => {
                    struct_ser.serialize_field("responseTrailers", v)?;
                }
                processing_response::Response::ImmediateResponse(v) => {
                    struct_ser.serialize_field("immediateResponse", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProcessingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_metadata",
            "dynamicMetadata",
            "mode_override",
            "modeOverride",
            "request_headers",
            "requestHeaders",
            "response_headers",
            "responseHeaders",
            "request_body",
            "requestBody",
            "response_body",
            "responseBody",
            "request_trailers",
            "requestTrailers",
            "response_trailers",
            "responseTrailers",
            "immediate_response",
            "immediateResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicMetadata,
            ModeOverride,
            RequestHeaders,
            ResponseHeaders,
            RequestBody,
            ResponseBody,
            RequestTrailers,
            ResponseTrailers,
            ImmediateResponse,
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
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            "modeOverride" | "mode_override" => Ok(GeneratedField::ModeOverride),
                            "requestHeaders" | "request_headers" => Ok(GeneratedField::RequestHeaders),
                            "responseHeaders" | "response_headers" => Ok(GeneratedField::ResponseHeaders),
                            "requestBody" | "request_body" => Ok(GeneratedField::RequestBody),
                            "responseBody" | "response_body" => Ok(GeneratedField::ResponseBody),
                            "requestTrailers" | "request_trailers" => Ok(GeneratedField::RequestTrailers),
                            "responseTrailers" | "response_trailers" => Ok(GeneratedField::ResponseTrailers),
                            "immediateResponse" | "immediate_response" => Ok(GeneratedField::ImmediateResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProcessingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.ProcessingResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProcessingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dynamic_metadata__ = None;
                let mut mode_override__ = None;
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DynamicMetadata => {
                            if dynamic_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            dynamic_metadata__ = map.next_value()?;
                        }
                        GeneratedField::ModeOverride => {
                            if mode_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modeOverride"));
                            }
                            mode_override__ = map.next_value()?;
                        }
                        GeneratedField::RequestHeaders => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaders"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(processing_response::Response::RequestHeaders)
;
                        }
                        GeneratedField::ResponseHeaders => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeaders"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(processing_response::Response::ResponseHeaders)
;
                        }
                        GeneratedField::RequestBody => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestBody"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(processing_response::Response::RequestBody)
;
                        }
                        GeneratedField::ResponseBody => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseBody"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(processing_response::Response::ResponseBody)
;
                        }
                        GeneratedField::RequestTrailers => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTrailers"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(processing_response::Response::RequestTrailers)
;
                        }
                        GeneratedField::ResponseTrailers => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTrailers"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(processing_response::Response::ResponseTrailers)
;
                        }
                        GeneratedField::ImmediateResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("immediateResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(processing_response::Response::ImmediateResponse)
;
                        }
                    }
                }
                Ok(ProcessingResponse {
                    dynamic_metadata: dynamic_metadata__,
                    mode_override: mode_override__,
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.ProcessingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TrailersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header_mutation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ext_proc.v3.TrailersResponse", len)?;
        if let Some(v) = self.header_mutation.as_ref() {
            struct_ser.serialize_field("headerMutation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TrailersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_mutation",
            "headerMutation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderMutation,
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
                            "headerMutation" | "header_mutation" => Ok(GeneratedField::HeaderMutation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TrailersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ext_proc.v3.TrailersResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TrailersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_mutation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HeaderMutation => {
                            if header_mutation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerMutation"));
                            }
                            header_mutation__ = map.next_value()?;
                        }
                    }
                }
                Ok(TrailersResponse {
                    header_mutation: header_mutation__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ext_proc.v3.TrailersResponse", FIELDS, GeneratedVisitor)
    }
}
