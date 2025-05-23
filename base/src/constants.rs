#![allow(dead_code)]

use crate::Marshalable;
use open_enum::open_enum;

pub const TPM2_SHA_DIGEST_SIZE: u32 = 20;
pub const TPM2_SHA1_DIGEST_SIZE: u32 = 20;
pub const TPM2_SHA256_DIGEST_SIZE: u32 = 32;
pub const TPM2_SHA384_DIGEST_SIZE: u32 = 48;
pub const TPM2_SHA512_DIGEST_SIZE: u32 = 64;
pub const TPM2_SM3_256_DIGEST_SIZE: u32 = 32;

pub const TPM2_MAX_DIGEST_BUFFER: u32 = 1024;
pub const TPM2_MAX_NV_BUFFER_SIZE: u32 = 2048;
pub const TPM2_MAX_CAP_BUFFER: u32 = 1024;
pub const TPM2_NUM_PCR_BANKS: u32 = 16;
pub const TPM2_MAX_PCRS: u32 = 32;
pub const TPM2_PCR_SELECT_MAX: u32 = TPM2_MAX_PCRS.div_ceil(8);
pub const TPM2_LABEL_MAX_BUFFER: u32 = 32;

/* Encryption block sizes */
pub const TPM2_MAX_SYM_BLOCK_SIZE: u32 = 16;
pub const TPM2_MAX_SYM_DATA: u32 = 256;
pub const TPM2_MAX_ECC_KEY_BYTES: u32 = 128;
pub const TPM2_MAX_SYM_KEY_BYTES: u32 = 32;
pub const TPM2_MAX_RSA_KEY_BYTES: u32 = 512;

pub const TPM2_PRIVATE_VENDOR_SPECIFIC_BYTES: u32 = (TPM2_MAX_RSA_KEY_BYTES / 2) * (3 + 2);

pub const TPM2_MAX_CONTEXT_SIZE: u32 = 5120;
pub const TPM2_MAX_ACTIVE_SESSIONS: u32 = 64;

// TpmAlgId represents a TPM_ALG_ID.
// See definition in Part 2: Structures, section 6.3.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmAlgId {
    RSA = 0x0001,
    TDES = 0x0003,
    SHA1 = 0x0004,
    HMAC = 0x0005,
    AES = 0x0006,
    MGF1 = 0x0007,
    KeyedHash = 0x0008,
    XOR = 0x000A,
    SHA256 = 0x000B,
    SHA384 = 0x000C,
    SHA512 = 0x000D,
    Null = 0x0010,
    SM3256 = 0x0012,
    SM4 = 0x0013,
    RSASSA = 0x0014,
    RSAES = 0x0015,
    RSAPSS = 0x0016,
    OAEP = 0x0017,
    ECDSA = 0x0018,
    ECDH = 0x0019,
    ECDAA = 0x001A,
    SM2 = 0x001B,
    ECSchnorr = 0x001C,
    ECMQV = 0x001D,
    KDF1SP80056A = 0x0020,
    KDF2 = 0x0021,
    KDF1SP800108 = 0x0022,
    ECC = 0x0023,
    SymCipher = 0x0025,
    Camellia = 0x0026,
    SHA3256 = 0x0027,
    SHA3384 = 0x0028,
    SHA3512 = 0x0029,
    CMAC = 0x003F,
    CTR = 0x0040,
    OFB = 0x0041,
    CBC = 0x0042,
    CFB = 0x0043,
    ECB = 0x0044,
}

// TpmEccCurve represents a TPM_ECC_Curve.
// See definition in Part 2: Structures, section 6.4.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmEccCurve {
    None = 0x0000,
    NistP192 = 0x0001,
    NistP224 = 0x0002,
    NistP256 = 0x0003,
    NistP384 = 0x0004,
    NistP521 = 0x0005,
    BNP256 = 0x0010,
    BNP638 = 0x0011,
    SM2P256 = 0x0020,
}

// The TPM_CC command codes
// See definition in Part 2: Structures, section 6.5.2.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmCc {
    NVUndefineSpaceSpecial = 0x0000011F,
    EvictControl = 0x00000120,
    HierarchyControl = 0x00000121,
    NVUndefineSpace = 0x00000122,
    ChangeEPS = 0x00000124,
    ChangePPS = 0x00000125,
    Clear = 0x00000126,
    ClearControl = 0x00000127,
    ClockSet = 0x00000128,
    HierarchyChanegAuth = 0x00000129,
    NVDefineSpace = 0x0000012A,
    PCRAllocate = 0x0000012B,
    PCRSetAuthPolicy = 0x0000012C,
    PPCommands = 0x0000012D,
    SetPrimaryPolicy = 0x0000012E,
    FieldUpgradeStart = 0x0000012F,
    ClockRateAdjust = 0x00000130,
    CreatePrimary = 0x00000131,
    NVGlobalWriteLock = 0x00000132,
    GetCommandAuditDigest = 0x00000133,
    NVIncrement = 0x00000134,
    NVSetBits = 0x00000135,
    NVExtend = 0x00000136,
    NVWrite = 0x00000137,
    NVWriteLock = 0x00000138,
    DictionaryAttackLockReset = 0x00000139,
    DictionaryAttackParameters = 0x0000013A,
    NVChangeAuth = 0x0000013B,
    PCREvent = 0x0000013C,
    PCRReset = 0x0000013D,
    SequenceComplete = 0x0000013E,
    SetAlgorithmSet = 0x0000013F,
    SetCommandCodeAuditStatus = 0x00000140,
    FieldUpgradeData = 0x00000141,
    IncrementalSelfTest = 0x00000142,
    SelfTest = 0x00000143,
    Startup = 0x00000144,
    Shutdown = 0x00000145,
    StirRandom = 0x00000146,
    ActivateCredential = 0x00000147,
    Certify = 0x00000148,
    PolicyNV = 0x00000149,
    CertifyCreation = 0x0000014A,
    Duplicate = 0x0000014B,
    GetTime = 0x0000014C,
    GetSessionAuditDigest = 0x0000014D,
    NVRead = 0x0000014E,
    NVReadLock = 0x0000014F,
    ObjectChangeAuth = 0x00000150,
    PolicySecret = 0x00000151,
    Rewrap = 0x00000152,
    Create = 0x00000153,
    ECDHZGen = 0x00000154,
    MAC = 0x00000155,
    Import = 0x00000156,
    Load = 0x00000157,
    Quote = 0x00000158,
    RSADecrypt = 0x00000159,
    MACStart = 0x0000015B,
    SequenceUpdate = 0x0000015C,
    Sign = 0x0000015D,
    Unseal = 0x0000015E,
    PolicySigned = 0x00000160,
    ContextLoad = 0x00000161,
    ContextSave = 0x00000162,
    ECDHKeyGen = 0x00000163,
    EncryptDecrypt = 0x00000164,
    FlushContext = 0x00000165,
    LoadExternal = 0x00000167,
    MakeCredential = 0x00000168,
    NVReadPublic = 0x00000169,
    PolicyAuthorize = 0x0000016A,
    PolicyAuthValue = 0x0000016B,
    PolicyCommandCode = 0x0000016C,
    PolicyCounterTimer = 0x0000016D,
    PolicyCpHash = 0x0000016E,
    PolicyLocality = 0x0000016F,
    PolicyNameHash = 0x00000170,
    PolicyOR = 0x00000171,
    PolicyTicket = 0x00000172,
    ReadPublic = 0x00000173,
    RSAEncrypt = 0x00000174,
    StartAuthSession = 0x00000176,
    VerifySignature = 0x00000177,
    ECCParameters = 0x00000178,
    FirmwareRead = 0x00000179,
    GetCapability = 0x0000017A,
    /// Gets a random sequence of bytes (`TPM_CC_GetRandom`).
    GetRandom = 0x0000017B,
    GetTestResult = 0x0000017C,
    Hash = 0x0000017D,
    PCRRead = 0x0000017E,
    PolicyPCR = 0x0000017F,
    PolicyRestart = 0x00000180,
    ReadClock = 0x00000181,
    PCRExtend = 0x00000182,
    PCRSetAuthValue = 0x00000183,
    NVCertify = 0x00000184,
    EventSequenceComplete = 0x00000185,
    HashSequenceStart = 0x00000186,
    PolicyPhysicalPresence = 0x00000187,
    PolicyDuplicationSelect = 0x00000188,
    PolicyGetDigest = 0x00000189,
    TestParams = 0x0000018A,
    Commit = 0x0000018B,
    PolicyPassword = 0x0000018C,
    ZGen2Phase = 0x0000018D,
    ECEphemeral = 0x0000018E,
    PolicyNvWritten = 0x0000018F,
    PolicyTemplate = 0x00000190,
    CreateLoaded = 0x00000191,
    PolicyAuthorizeNV = 0x00000192,
    EncryptDecrypt2 = 0x00000193,
    ACGetCapability = 0x00000194,
    ACSend = 0x00000195,
    PolicyACSendSelect = 0x00000196,
    CertifyX509 = 0x00000197,
    ACTSetTimeout = 0x00000198,
}

// TpmRc represents a TPM_RC.
// See definition in Part 2: Structures, section 6.6.
#[open_enum]
#[repr(u32)]
pub enum TpmRc {
    Success = 0x00000000,
    // FMT0 error codes
    Initialize = TpmRc::RC_VER_1,
    Failure = TpmRc::RC_VER_1 + 0x001,
    Sequence = TpmRc::RC_VER_1 + 0x003,
    Private = TpmRc::RC_VER_1 + 0x00B,
    Hmac = TpmRc::RC_VER_1 + 0x019,
    Disabled = TpmRc::RC_VER_1 + 0x020,
    Exclusive = TpmRc::RC_VER_1 + 0x021,
    AuthType = TpmRc::RC_VER_1 + 0x024,
    AuthMissing = TpmRc::RC_VER_1 + 0x025,
    Policy = TpmRc::RC_VER_1 + 0x026,
    Pcr = TpmRc::RC_VER_1 + 0x027,
    PCRChanged = TpmRc::RC_VER_1 + 0x028,
    Upgrade = TpmRc::RC_VER_1 + 0x02D,
    TooManyContexts = TpmRc::RC_VER_1 + 0x02E,
    AuthUnavailable = TpmRc::RC_VER_1 + 0x02F,
    Reboot = TpmRc::RC_VER_1 + 0x030,
    Unbalanced = TpmRc::RC_VER_1 + 0x031,
    CommandSize = TpmRc::RC_VER_1 + 0x042,
    CommandCode = TpmRc::RC_VER_1 + 0x043,
    AuthSize = TpmRc::RC_VER_1 + 0x044,
    AuthContext = TpmRc::RC_VER_1 + 0x045,
    NVRange = TpmRc::RC_VER_1 + 0x046,
    NVSize = TpmRc::RC_VER_1 + 0x047,
    NVLocked = TpmRc::RC_VER_1 + 0x048,
    NVAuthorization = TpmRc::RC_VER_1 + 0x049,
    NVUninitialized = TpmRc::RC_VER_1 + 0x04A,
    NVSpace = TpmRc::RC_VER_1 + 0x04B,
    NVDefined = TpmRc::RC_VER_1 + 0x04C,
    BadContext = TpmRc::RC_VER_1 + 0x050,
    CPHash = TpmRc::RC_VER_1 + 0x051,
    Parent = TpmRc::RC_VER_1 + 0x052,
    NeedsTest = TpmRc::RC_VER_1 + 0x053,
    NoResult = TpmRc::RC_VER_1 + 0x054,
    Sensitive = TpmRc::RC_VER_1 + 0x055,
    // FMT1 error codes
    CAsymmetric = TpmRc::RC_FMT_1 + 0x001,
    Attributes = TpmRc::RC_FMT_1 + 0x002,
    Hash = TpmRc::RC_FMT_1 + 0x003,
    Value = TpmRc::RC_FMT_1 + 0x004,
    Hierarchy = TpmRc::RC_FMT_1 + 0x005,
    KeySize = TpmRc::RC_FMT_1 + 0x007,
    Mgf = TpmRc::RC_FMT_1 + 0x008,
    Mode = TpmRc::RC_FMT_1 + 0x009,
    Type = TpmRc::RC_FMT_1 + 0x00A,
    Handle = TpmRc::RC_FMT_1 + 0x00B,
    Kdf = TpmRc::RC_FMT_1 + 0x00C,
    Range = TpmRc::RC_FMT_1 + 0x00D,
    AuthFail = TpmRc::RC_FMT_1 + 0x00E,
    Nonce = TpmRc::RC_FMT_1 + 0x00F,
    PP = TpmRc::RC_FMT_1 + 0x010,
    Scheme = TpmRc::RC_FMT_1 + 0x012,
    Size = TpmRc::RC_FMT_1 + 0x015,
    Symmetric = TpmRc::RC_FMT_1 + 0x016,
    Tag = TpmRc::RC_FMT_1 + 0x017,
    Selector = TpmRc::RC_FMT_1 + 0x018,
    Insufficient = TpmRc::RC_FMT_1 + 0x01A,
    Signature = TpmRc::RC_FMT_1 + 0x01B,
    Key = TpmRc::RC_FMT_1 + 0x01C,
    PolicyFail = TpmRc::RC_FMT_1 + 0x01D,
    Integrity = TpmRc::RC_FMT_1 + 0x01F,
    Ticket = TpmRc::RC_FMT_1 + 0x020,
    ReservedBits = TpmRc::RC_FMT_1 + 0x021,
    BadAuth = TpmRc::RC_FMT_1 + 0x022,
    Expired = TpmRc::RC_FMT_1 + 0x023,
    PolicyCC = TpmRc::RC_FMT_1 + 0x024,
    Binding = TpmRc::RC_FMT_1 + 0x025,
    Curve = TpmRc::RC_FMT_1 + 0x026,
    ECCPoint = TpmRc::RC_FMT_1 + 0x027,
    // Warnings
    ContextGap = TpmRc::RC_WARN + 0x001,
    ObjectMemory = TpmRc::RC_WARN + 0x002,
    SessionMemory = TpmRc::RC_WARN + 0x003,
    Memory = TpmRc::RC_WARN + 0x004,
    SessionHandles = TpmRc::RC_WARN + 0x005,
    ObjectHandles = TpmRc::RC_WARN + 0x006,
    Locality = TpmRc::RC_WARN + 0x007,
    Yielded = TpmRc::RC_WARN + 0x008,
    Canceled = TpmRc::RC_WARN + 0x009,
    Testing = TpmRc::RC_WARN + 0x00A,
    ReferenceH0 = TpmRc::RC_WARN + 0x010,
    ReferenceH1 = TpmRc::RC_WARN + 0x011,
    ReferenceH2 = TpmRc::RC_WARN + 0x012,
    ReferenceH3 = TpmRc::RC_WARN + 0x013,
    ReferenceH4 = TpmRc::RC_WARN + 0x014,
    ReferenceH5 = TpmRc::RC_WARN + 0x015,
    ReferenceH6 = TpmRc::RC_WARN + 0x016,
    ReferenceS0 = TpmRc::RC_WARN + 0x018,
    ReferenceS1 = TpmRc::RC_WARN + 0x019,
    ReferenceS2 = TpmRc::RC_WARN + 0x01A,
    ReferenceS3 = TpmRc::RC_WARN + 0x01B,
    ReferenceS4 = TpmRc::RC_WARN + 0x01C,
    ReferenceS5 = TpmRc::RC_WARN + 0x01D,
    ReferenceS6 = TpmRc::RC_WARN + 0x01E,
    NVRate = TpmRc::RC_WARN + 0x020,
    Lockout = TpmRc::RC_WARN + 0x021,
    Retry = TpmRc::RC_WARN + 0x022,
    NVUnavailable = TpmRc::RC_WARN + 0x023,
}

impl TpmRc {
    pub const RC_VER_1: u32 = 0x00000100;
    pub const RC_FMT_1: u32 = 0x00000080;
    pub const RC_WARN: u32 = 0x00000900;
    pub const RC_P: u32 = 0x00000040;
    pub const RC_S: u32 = 0x00000800;
}

// TpmEo represents a TPM_EO.
// See definition in Part 2: Structures, section 6.8.
#[open_enum]
#[repr(u16)]
pub enum TpmEo {
    Eq = 0x0000,
    Neq = 0x0001,
    SignedGT = 0x0002,
    UnsignedGT = 0x0003,
    SignedLT = 0x0004,
    UnsignedLT = 0x0005,
    SignedGE = 0x0006,
    UnsignedGE = 0x0007,
    SignedLE = 0x0008,
    UnsignedLE = 0x0009,
    BitSet = 0x000A,
    BitClear = 0x000B,
}

// TpmSt represents a TPM_ST.
// See definition in Part 2: Structures, section 6.9.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmSt {
    RspCommand = 0x00C4,
    Null = 0x8000,
    NoSessions = 0x8001,
    Sessions = 0x8002,
    AttestNV = 0x8014,
    AttestCommandAudit = 0x8015,
    AttestSessionAudit = 0x8016,
    AttestCertify = 0x8017,
    AttestQuote = 0x8018,
    AttestTime = 0x8019,
    AttestCreation = 0x801A,
    AttestNVDigest = 0x801C,
    Creation = 0x8021,
    Verified = 0x8022,
    AuthSecret = 0x8023,
    HashCheck = 0x8024,
    AuthSigned = 0x8025,
    FuManifest = 0x8029,
}

// TpmSu represents a TPM_SU.
// See definition in Part 2: Structures, section 6.10.
#[open_enum]
#[repr(u16)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmSu {
    Clear = 0x0000,
    State = 0x0001,
}

// TpmSe represents a TPM_SE.
// See definition in Part 2: Structures, section 6.11.
#[open_enum]
#[repr(u8)]
pub enum TpmSe {
    HMAC = 0x00,
    Policy = 0x01,
    Trial = 0x03,
}

// TpmCap represents a TPM_CAP.
// See definition in Part 2: Structures, section 6.12
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmCap {
    Algs = 0x00000000,
    Handles = 0x00000001,
    Commands = 0x00000002,
    PPCommands = 0x00000003,
    AuditCommands = 0x00000004,
    PCRs = 0x00000005,
    TPMProperties = 0x00000006,
    PCRProperties = 0x00000007,
    ECCCurves = 0x00000008,
    AuthPolicies = 0x00000009,
    ACT = 0x0000000A,
}

// TpmPt represents a TPM_PT.
// See definition in Part 2: Structures, section 6.13.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmPt {
    // a 4-octet character string containing the TPM Family value
    // (TPM_SPEC_FAMILY)
    FamilyIndicator = 0x00000100,
    // the level of the specification
    Level = 0x00000101,
    // the specification Revision times 100
    Revision = 0x00000102,
    // the specification day of year using TCG calendar
    DayofYear = 0x00000103,
    // the specification year using the CE
    Year = 0x00000104,
    // the vendor ID unique to each TPM manufacturer
    Manufacturer = 0x00000105,
    // the first four characters of the vendor ID string
    VendorString1 = 0x00000106,
    // the second four characters of the vendor ID string
    VendorString2 = 0x00000107,
    // the third four characters of the vendor ID string
    VendorString3 = 0x00000108,
    // the fourth four characters of the vendor ID sting
    VendorString4 = 0x00000109,
    // vendor-defined value indicating the TPM model
    VendorTPMType = 0x0000010A,
    // the most-significant 32 bits of a TPM vendor-specific value
    // indicating the version number of the firmware.
    FirmwareVersion1 = 0x0000010B,
    // the least-significant 32 bits of a TPM vendor-specific value
    // indicating the version number of the firmware.
    FirmwareVersion2 = 0x0000010C,
    // the maximum size of a parameter TPM2B_MAX_BUFFER)
    InputBuffer = 0x0000010D,
    // the minimum number of transient objects that can be held in TPM RAM
    HRTransientMin = 0x0000010E,
    // the minimum number of persistent objects that can be held in TPM NV
    // memory
    HRPersistentMin = 0x0000010F,
    // the minimum number of authorization sessions that can be held in TPM
    // RAM
    HRLoadedMin = 0x00000110,
    // the number of authorization sessions that may be active at a time
    ActiveSessionsMax = 0x00000111,
    // the number of PCR implemented
    PCRCount = 0x00000112,
    // the minimum number of octets in a TPMS_PCR_SELECT.sizeOfSelect
    PCRSelectMin = 0x00000113,
    // the maximum allowed difference (unsigned) between the contextID
    // values of two saved session contexts
    ContextGapMax = 0x00000114,
    // the maximum number of NV Indexes that are allowed to have the
    // TPM_NT_COUNTER attribute
    NVCountersMax = 0x00000116,
    // the maximum size of an NV Index data area
    NVIndexMax = 0x00000117,
    // a TPMA_MEMORY indicating the memory management method for the TPM
    Memory = 0x00000118,
    // interval, in milliseconds, between updates to the copy of
    // TPMS_CLOCK_INFO.clock in NV
    ClockUpdate = 0x00000119,
    // the algorithm used for the integrity HMAC on saved contexts and for
    // hashing the fuData of TPM2_FirmwareRead()
    ContextHash = 0x0000011A,
    // TPM_ALG_ID, the algorithm used for encryption of saved contexts
    ContextSym = 0x0000011B,
    // TPM_KEY_BITS, the size of the key used for encryption of saved
    // contexts
    ContextSymSize = 0x0000011C,
    // the modulus - 1 of the count for NV update of an orderly counter
    OrderlyCount = 0x0000011D,
    // the maximum value for commandSize in a command
    MaxCommandSize = 0x0000011E,
    // the maximum value for responseSize in a response
    MaxResponseSize = 0x0000011F,
    // the maximum size of a digest that can be produced by the TPM
    MaxDigest = 0x00000120,
    // the maximum size of an object context that will be returned by
    // TPM2_ContextSave
    MaxObjectContext = 0x00000121,
    // the maximum size of a session context that will be returned by
    // TPM2_ContextSave
    MaxSessionContext = 0x00000122,
    // platform-specific family (a TPM_PS value)(see Table 25)
    PSFamilyIndicator = 0x00000123,
    // the level of the platform-specific specification
    PSLevel = 0x00000124,
    // a platform specific value
    PSRevision = 0x00000125,
    // the platform-specific TPM specification day of year using TCG
    // calendar
    PSDayOfYear = 0x00000126,
    // the platform-specific TPM specification year using the CE
    PSYear = 0x00000127,
    // the number of split signing operations supported by the TPM
    SplitMax = 0x00000128,
    // total number of commands implemented in the TPM
    TotalCommands = 0x00000129,
    // number of commands from the TPM library that are implemented
    LibraryCommands = 0x0000012A,
    // number of vendor commands that are implemented
    VendorCommands = 0x0000012B,
    // the maximum data size in one NV write, NV read, NV extend, or NV
    // certify command
    NVBufferMax = 0x0000012C,
    // a TPMA_MODES value, indicating that the TPM is designed for these
    // modes.
    Modes = 0x0000012D,
    // the maximum size of a TPMS_CAPABILITY_DATA structure returned in
    // TPM2_GetCapability().
    MaxCapBuffer = 0x0000012E,
    // TPMA_PERMANENT
    Permanent = 0x00000200,
    // TPMA_STARTUP_CLEAR
    StartupClear = 0x00000201,
    // the number of NV Indexes currently defined
    HRNVIndex = 0x00000202,
    // the number of authorization sessions currently loaded into TPM RAM
    HRLoaded = 0x00000203,
    // the number of additional authorization sessions, of any type, that
    // could be loaded into TPM RAM
    HRLoadedAvail = 0x00000204,
    // the number of active authorization sessions currently being tracked
    // by the TPM
    HRActive = 0x00000205,
    // the number of additional authorization sessions, of any type, that
    // could be created
    HRActiveAvail = 0x00000206,
    // estimate of the number of additional transient objects that could be
    // loaded into TPM RAM
    HRTransientAvail = 0x00000207,
    // the number of persistent objects currently loaded into TPM NV memory
    HRPersistent = 0x00000208,
    // the number of additional persistent objects that could be loaded into
    // NV memory
    HRPersistentAvail = 0x00000209,
    // the number of defined NV Indexes that have NV the TPM_NT_COUNTER
    // attribute
    NVCounters = 0x0000020A,
    // the number of additional NV Indexes that can be defined with their
    // TPM_NT of TPM_NV_COUNTER and the TPMA_NV_ORDERLY attribute SET
    NVCountersAvail = 0x0000020B,
    // code that limits the algorithms that may be used with the TPM
    AlgorithmSet = 0x0000020C,
    // the number of loaded ECC curves
    LoadedCurves = 0x0000020D,
    // the current value of the lockout counter (failedTries)
    LockoutCounter = 0x0000020E,
    // the number of authorization failures before DA lockout is invoked
    MaxAuthFail = 0x0000020F,
    // the number of seconds before the value reported by
    // TPM_PT_LOCKOUT_COUNTER is decremented
    LockoutInterval = 0x00000210,
    // the number of seconds after a lockoutAuth failure before use of
    // lockoutAuth may be attempted again
    LockoutRecovery = 0x00000211,
    // number of milliseconds before the TPM will accept another command
    // that will modify NV
    NVWriteRecovery = 0x00000212,
    // the high-order 32 bits of the command audit counter
    AuditCounter0 = 0x00000213,
    // the low-order 32 bits of the command audit counter
    AuditCounter1 = 0x00000214,
}

// TpmPtPcr represents a TPM_PT_PCR.
// See definition in Part 2: Structures, section 6.14.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmPtPcr {
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR is saved and
    // restored by TPM_SU_STATE
    Save = 0x00000000,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 0
    ExtendL0 = 0x00000001,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 0
    ResetL0 = 0x00000002,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 1
    ExtendL1 = 0x00000003,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 1
    ResetL1 = 0x00000004,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 2
    ExtendL2 = 0x00000005,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 2
    ResetL2 = 0x00000006,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 3
    ExtendL3 = 0x00000007,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 3
    ResetL3 = 0x00000008,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be
    // extended from locality 4
    ExtendL4 = 0x00000009,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset
    // by TPM2_PCR_Reset() from locality 4
    ResetL4 = 0x0000000A,
    // a SET bit in the TPMS_PCR_SELECT indicates that modifications to this
    // PCR (reset or Extend) will not increment the pcrUpdateCounter
    NoIncrement = 0x00000011,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR is reset by a
    // D-RTM event
    DRTMRest = 0x00000012,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR is controlled
    // by policy
    Policy = 0x00000013,
    // a SET bit in the TPMS_PCR_SELECT indicates that the PCR is controlled
    // by an authorization value
    Auth = 0x00000014,
}

// TpmHt represents a TPM_HT.
// See definition in Part 2: Structures, section 7.2.
#[open_enum]
#[repr(u8)]
pub enum TpmHt {
    PCR = 0x00,
    NVIndex = 0x01,
    HMACSession = 0x02,
    PolicySession = 0x03,
    Permanent = 0x40,
    Transient = 0x80,
    Persistent = 0x81,
    AC = 0x90,
}

// TpmHandle represents a TPM_HANDLE.
// See definition in Part 2: Structures, section 7.1.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmHandle {
    RHOwner = 0x40000001,
    RHNull = 0x40000007,
    RSPW = 0x40000009,
    RHLockout = 0x4000000A,
    RHEndorsement = 0x4000000B,
    RHPlatform = 0x4000000C,
    RHPlatformNV = 0x4000000D,
}
/// TpmHc represents a TPM_HC.
/// See definition in Part 2: Structures, section 7.5.
#[derive(Copy, Clone, Debug, Default, Marshalable)]
pub struct TpmHc(u32);
#[allow(non_upper_case_globals)]
impl TpmHc {
    /// Masks off the HR.
    const HRHandleMask: u32 = 0x00FFFFFF;
    /// Masks off the variable part.
    const HRRangeMask: u32 = 0xFF000000;
    /// Handle constant shift.
    const HRShift: u32 = 24;

    const fn new(handle_type: TpmHt) -> TpmHc {
        TpmHc((handle_type.0 as u32) << TpmHc::HRShift)
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    /// PCR handle range base.
    const HRPcr: TpmHc = TpmHc::new(TpmHt::PCR);
    /// HMAC session handle range base.
    const HRHMACSession: TpmHc = TpmHc::new(TpmHt::HMACSession);
    /// Policy session handle range base.
    const HRPolicySession: TpmHc = TpmHc::new(TpmHt::PolicySession);
    /// Transient object handle range base.
    const HRTransient: TpmHc = TpmHc::new(TpmHt::Transient);
    /// Persistent object handle range base.
    const HRPersistent: TpmHc = TpmHc::new(TpmHt::Persistent);
    /// NV index handle range base.
    const HRNvIndex: TpmHc = TpmHc::new(TpmHt::NVIndex);
    // TODO: Add remaining values and ranges, some of which are profile-dependent.

    /// The first HMAC session.
    pub const HmacSessionFirst: TpmHc = TpmHc::HRHMACSession;
    /// The last HMAC session.
    pub const HmacSessionLast: TpmHc = TpmHc(TpmHc::HRHMACSession.0 + TPM2_MAX_ACTIVE_SESSIONS - 1);
    /// Returns true if the value is a valid HMAC session handle.
    pub fn is_hmac_session(value: u32) -> bool {
        (TpmHc::HmacSessionFirst.0..=TpmHc::HmacSessionLast.0).contains(&value)
    }
    /// The first policy session.
    pub const PolicySessionFirst: TpmHc = TpmHc::HRPolicySession;
    /// The last policy session.
    pub const PolicySessionLast: TpmHc =
        TpmHc(TpmHc::HRPolicySession.0 + TPM2_MAX_ACTIVE_SESSIONS - 1);
    /// Returns true if the value is a valid policy session handle.
    pub fn is_policy_session(value: u32) -> bool {
        (TpmHc::PolicySessionFirst.0..=TpmHc::PolicySessionLast.0).contains(&value)
    }
    /// The first persistent object.
    pub const PersistentFirst: TpmHc = TpmHc::HRPersistent;
    /// The last persistent object.
    pub const PersistentLast: TpmHc = TpmHc(TpmHc::HRPersistent.0 + 0x00FFFFFF);
    /// The first allowed NV index.
    pub const NVIndexFirst: TpmHc = TpmHc::HRNvIndex;
    /// The last allowed NV index.
    pub const NVIndexLast: TpmHc = TpmHc(TpmHc::NVIndexFirst.0 + 0x00FFFFFF);
    /// Returns true if the value is an allowed NV index.
    pub fn is_nv_index(value: u32) -> bool {
        (TpmHc::NVIndexFirst.0..=TpmHc::NVIndexLast.0).contains(&value)
    }
}

// TpmNt represents a TPM_NT.
// See definition in Part 2: Structures, section 13.4.
#[open_enum]
#[repr(u8)]
pub enum TpmNt {
    // contains data that is opaque to the TPM that can only be modified
    // using TPM2_NV_Write().
    Ordinary = 0x0,
    // contains an 8-octet value that is to be used as a counter and can
    // only be modified with TPM2_NV_Increment()
    Counter = 0x1,
    // contains an 8-octet value to be used as a bit field and can only be
    // modified with TPM2_NV_SetBits().
    Bits = 0x2,
    // contains a digest-sized value used like a PCR. The Index can only be
    // modified using TPM2_NV_Extend(). The extend will use the nameAlg of
    // the Index.
    Extend = 0x4,
    // contains pinCount that increments on a PIN authorization failure and
    // a pinLimit
    PinFail = 0x8,
    // contains pinCount that increments on a PIN authorization success and
    // a pinLimit
    PinPass = 0x9,
}

/// TpmGenerated represents a TPM_GENERATED.
/// See definition in Part 2: Structures, section 6.2.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TpmGenerated {
    VALUE = 0xFF544347,
}
