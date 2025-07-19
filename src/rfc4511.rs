/// LDAP Result Codes as defined in RFC 4511 Appendix A.
/// See: https://www.rfc-editor.org/rfc/rfc4511#appendix-A
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LdapResultCode {
    /// The operation completed successfully.
    Success = 0,

    /// The client must bind before attempting other operations.
    OperationsError = 1,

    ProtocolError = 2,
    /// The requested operation is not supported.
    TimeLimitExceeded = 3,
    /// The requested operation exceeded a size limit.
    SizeLimitExceeded = 4,
    /// The operation cannot be performed because the client is not authenticated.
    CompareFalse = 5,
    /// The compare operation failed.
    CompareTrue = 6,
    /// Authentication failed due to invalid credentials.
    AuthMethodNotSupported = 7,
    /// The server is unwilling to perform the operation.
    StrongAuthRequired = 8,
    /// Referral for the requested operation.
    Referral = 9,
    /// The client is not authorized to perform the requested operation.
    AdminLimitExceeded = 11,
    /// The requested operation is unavailable.
    UnavailableCriticalExtension = 12,
    /// The operation is ignored.
    ConfidentialityRequired = 13,
    /// The client requests more information.
    SaslBindInProgress = 14,
    /// The requested operation will be performed later.
    NoSuchAttribute = 16,
    /// The attribute does not exist.
    UndefinedAttributeType = 17,
    /// The attribute type is undefined.
    InappropriateMatching = 18,
    /// The requested matching rule is inappropriate.
    ConstraintViolation = 19,
    /// The operation violated a constraint.
    AttributeOrValueExists = 20,
    /// The attribute or value already exists.
    InvalidAttributeSyntax = 21,
    /// The syntax of the attribute is invalid.
    NoSuchObject = 32,
    /// The requested object does not exist.
    AliasProblem = 33,
    /// The alias problem encountered.
    InvalidDNSyntax = 34,
    /// The distinguished name is invalid.
    AliasDereferencingProblem = 36,
    /// Problem dereferencing an alias.
    InappropriateAuthentication = 48,
    /// The authentication method is inappropriate.
    InvalidCredentials = 49,
    /// Invalid credentials provided.
    InsufficientAccessRights = 50,
    /// The client has insufficient access rights.
    Busy = 51,
    /// The server is busy.
    Unavailable = 52,
    /// The server is unavailable.
    UnwillingToPerform = 53,
    /// The server is unwilling to perform the operation.
    LoopDetect = 54,
    /// A loop was detected.
    NamingViolation = 64,
    /// Naming violation occurred.
    ObjectClassViolation = 65,
    /// Object class violation occurred.
    NotAllowedOnNonLeaf = 66,
    /// Operation not allowed on non-leaf object.
    NotAllowedOnRDN = 67,
    /// Operation not allowed on RDN.
    EntryAlreadyExists = 68,
    /// The entry already exists.
    ObjectClassModsProhibited = 69,
    /// Modifications to the object class are prohibited.
    ResultsTooLarge = 70,
    /// The results are too large.
    AffectsMultipleDSAs = 71,
    /// The operation affects multiple DSAs.
    // 80-90 are reserved for API specific errors; omitted here.
    // Standard codes continue:
    Other = 80,
    // There are other result codes such as 81-90 used in extended or vendor-specific contexts.
}

impl LdapResultCode {
    /// Try to convert a numeric LDAP result code to the enum variant.
    /// Returns None if the code is unknown.
    pub fn from_u32(code: u32) -> Option<Self> {
        match code {
            0 => Some(LdapResultCode::Success),
            1 => Some(LdapResultCode::OperationsError),
            2 => Some(LdapResultCode::ProtocolError),
            3 => Some(LdapResultCode::TimeLimitExceeded),
            4 => Some(LdapResultCode::SizeLimitExceeded),
            5 => Some(LdapResultCode::CompareFalse),
            6 => Some(LdapResultCode::CompareTrue),
            7 => Some(LdapResultCode::AuthMethodNotSupported),
            8 => Some(LdapResultCode::StrongAuthRequired),
            9 => Some(LdapResultCode::Referral),
            11 => Some(LdapResultCode::AdminLimitExceeded),
            12 => Some(LdapResultCode::UnavailableCriticalExtension),
            13 => Some(LdapResultCode::ConfidentialityRequired),
            14 => Some(LdapResultCode::SaslBindInProgress),
            16 => Some(LdapResultCode::NoSuchAttribute),
            17 => Some(LdapResultCode::UndefinedAttributeType),
            18 => Some(LdapResultCode::InappropriateMatching),
            19 => Some(LdapResultCode::ConstraintViolation),
            20 => Some(LdapResultCode::AttributeOrValueExists),
            21 => Some(LdapResultCode::InvalidAttributeSyntax),
            32 => Some(LdapResultCode::NoSuchObject),
            33 => Some(LdapResultCode::AliasProblem),
            34 => Some(LdapResultCode::InvalidDNSyntax),
            36 => Some(LdapResultCode::AliasDereferencingProblem),
            48 => Some(LdapResultCode::InappropriateAuthentication),
            49 => Some(LdapResultCode::InvalidCredentials),
            50 => Some(LdapResultCode::InsufficientAccessRights),
            51 => Some(LdapResultCode::Busy),
            52 => Some(LdapResultCode::Unavailable),
            53 => Some(LdapResultCode::UnwillingToPerform),
            54 => Some(LdapResultCode::LoopDetect),
            64 => Some(LdapResultCode::NamingViolation),
            65 => Some(LdapResultCode::ObjectClassViolation),
            66 => Some(LdapResultCode::NotAllowedOnNonLeaf),
            67 => Some(LdapResultCode::NotAllowedOnRDN),
            68 => Some(LdapResultCode::EntryAlreadyExists),
            69 => Some(LdapResultCode::ObjectClassModsProhibited),
            70 => Some(LdapResultCode::ResultsTooLarge),
            71 => Some(LdapResultCode::AffectsMultipleDSAs),
            80 => Some(LdapResultCode::Other),
            _ => None,
        }
    }

    /// Get human-readable description of the LDAP result code
    pub fn description(&self) -> &'static str {
        match self {
            LdapResultCode::Success => "Success",
            LdapResultCode::OperationsError => "Operations Error",
            LdapResultCode::ProtocolError => "Protocol Error",
            LdapResultCode::TimeLimitExceeded => "Time Limit Exceeded",
            LdapResultCode::SizeLimitExceeded => "Size Limit Exceeded",
            LdapResultCode::CompareFalse => "Compare False",
            LdapResultCode::CompareTrue => "Compare True",
            LdapResultCode::AuthMethodNotSupported => "Auth Method Not Supported",
            LdapResultCode::StrongAuthRequired => "Strong Authentication Required",
            LdapResultCode::Referral => "Referral",
            LdapResultCode::AdminLimitExceeded => "Admin Limit Exceeded",
            LdapResultCode::UnavailableCriticalExtension => "Unavailable Critical Extension",
            LdapResultCode::ConfidentialityRequired => "Confidentiality Required",
            LdapResultCode::SaslBindInProgress => "SASL Bind In Progress",
            LdapResultCode::NoSuchAttribute => "No Such Attribute",
            LdapResultCode::UndefinedAttributeType => "Undefined Attribute Type",
            LdapResultCode::InappropriateMatching => "Inappropriate Matching",
            LdapResultCode::ConstraintViolation => "Constraint Violation",
            LdapResultCode::AttributeOrValueExists => "Attribute Or Value Exists",
            LdapResultCode::InvalidAttributeSyntax => "Invalid Attribute Syntax",
            LdapResultCode::NoSuchObject => "No Such Object",
            LdapResultCode::AliasProblem => "Alias Problem",
            LdapResultCode::InvalidDNSyntax => "Invalid DN Syntax",
            LdapResultCode::AliasDereferencingProblem => "Alias Dereferencing Problem",
            LdapResultCode::InappropriateAuthentication => "Inappropriate Authentication",
            LdapResultCode::InvalidCredentials => "Invalid Credentials",
            LdapResultCode::InsufficientAccessRights => "Insufficient Access Rights",
            LdapResultCode::Busy => "Busy",
            LdapResultCode::Unavailable => "Unavailable",
            LdapResultCode::UnwillingToPerform => "Unwilling To Perform",
            LdapResultCode::LoopDetect => "Loop Detect",
            LdapResultCode::NamingViolation => "Naming Violation",
            LdapResultCode::ObjectClassViolation => "Object Class Violation",
            LdapResultCode::NotAllowedOnNonLeaf => "Not Allowed On Non-Leaf",
            LdapResultCode::NotAllowedOnRDN => "Not Allowed On RDN",
            LdapResultCode::EntryAlreadyExists => "Entry Already Exists",
            LdapResultCode::ObjectClassModsProhibited => "Object Class Mods Prohibited",
            LdapResultCode::ResultsTooLarge => "Results Too Large",
            LdapResultCode::AffectsMultipleDSAs => "Affects Multiple DSAs",
            LdapResultCode::Other => "Other",
        }
    }
}
