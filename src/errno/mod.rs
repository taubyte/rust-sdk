mod error_strings;
use error_strings::ERROR_STRINGS;

/*
    To generate updated error strings
        - even though this is a rust package, go will need to be installed.
        - note if office-space + go workspace office-space will not add the generate package into work space
            - make sure to kill go workspace `$ asd work d`.

    $ cd rust-sdk/src/errno/generate
    $ go run .
*/

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Errno {
    ErrorNone,
    ErrorEventNotFound,
    ErrorBufferTooSmall,
    ErrorAddressOutOfMemory,
    ErrorNilAddress,
    ErrorHttpWrite,
    ErrorHttpReadBody,
    ErrorCloseBody,
    ErrorEOF,
    ErrorReadHeaders,
    ErrorClientNotFound,
    ErrorParseUrlFailed,
    ErrorMemoryWriteFailed,
    ErrorHttpRequestFailed,
    ErrorInvalidMethod,
    ErrorNewRequestFailed,
    ErrorHeaderNotFound,
    ErrorHttpWriteBodyFailed,
    ErrorCidNotFoundOnIpfs,
    ErrorInvalidCid,
    ErrorSubscribeFailed,
    ErrorPublishFailed,
    ErrorDatabaseCreateFailed,
    ErrorDatabaseGetFailed,
    ErrorDatabasePutFailed,
    ErrorDatabaseNotFound,
    ErrorDatabaseDeleteFailed,
    ErrorDatabaseListFailed,
    ErrorKeystoreCreateFailed,
    ErrorDatabaseKeyNotFound,
    ErrorKeystoreNotFound,
    ErrorAddFileFailed,
    ErrorGetFileFailed,
    ErrorDeleteFileFailed,
    ErrorCloseFileFailed,
    ErrorFileNameNotFound,
    ErrorListFileVersionsFailed,
    ErrorListingUsedSpaceFailed,
    ErrorGetWebSocketURLFailed,
    ErrorByteConversionFailed,
    ErrorChannelNotFound,
    ErrorNewStreamFailed,
    ErrorCommandCreateFailed,
    ErrorP2PSendFailed,
    ErrorP2PCommandNotFound,
    ErrorP2PProtocolNotFound,
    ErrorP2PFromNotFound,
    ErrorP2PToNotFound,
    ErrorP2PListenFailed,
    ErrorMarshalDataFailed,
    ErrorZeroSize,
    ErrorEthereumNewClient,
    ErrorEthereumBlockNotFound,
    ErrorEthereumChainIdNotFound,
    ErrorEthereumInvalidHexKey,
    ErrorEthereumNonceNotFound,
    ErrorEthereumGasPriceNotFound,
    ErrorEthereumGasTipCapNotFound,
    ErrorEthereumGasFeeCapNotFound,
    ErrorEthereumGasNotFound,
    ErrorEthereumValueNotFound,
    ErrorEthereumDataNotFound,
    ErrorEthereumAddressNotFound,
    ErrorEthereumChainNotFound,
    ErrorEthereumHashNotFound,
    ErrorEthereumTransactionNotFound,
    ErrorEthereumSendTransactionFailed,
    ErrorEthereumMarshalJSON,
    ErrorEthereumMethodNotSupported,
    ErrorConvertibleConversionFailed,
    ErrorEthereumContractNotFound,
    ErrorEthereumParsingAbiFailed,
    ErrorEthereumParsingECDSAFailed,
    ErrorEthereumBindTransactorFailed,
    ErrorEthereumCallContractFailed,
    ErrorEthereumParseInputTypeFailed,
    ErrorEthereumParseOutputTypeFailed,
    ErrorEthereumContractMethodNotFound,
    ErrorEthereumInvalidContractMethodInput,
    ErrorEthereumInvalidContractMethodOutput,
    ErrorEthereumUnsupportedDataType,
    ErrorEthereumCannotCallPaidMutatorTransaction,
    ErrorEthereumCannotTransactFreeMethod,
    ErrorEthereumTransactMethodFailed,
    ErrorEthereumDeployFailed,
    ErrorEthereumSignFailed,
    ErrorEthereumInvalidPublicKey,
    ErrorEthereumInvalidPrivateKey,
    ErrorEthereumRecoverPubKeyFailed,
    ErrorSizeMismatch,
    ErrorStorageGetMetaFailed,
    ErrorAddFileToIpfsFailed,
    ErrorStorageNotFound,
    ErrorStorageListFailed,
    ErrorCreatingNewFile,
    ErrorWritingFile,
    ErrorContentNotFound,
    ErrorReadingFile,
    ErrorInvalidWhence,
    ErrorSeekingFile,
    ErrorCidNotFound,
    ErrorResolverNotFound,
    ErrorFailedTxTLookup,
    ErrorFailedAddressLookup,
    ErrorFailedCNAMELookup,
    ErrorFailedMXLookup,
    ErrorCachedResponseTypeNotFound,
    ErrorCachedResponseNotFound,
    SmartOpErrorResourceNotFound,
    SmartOpErrorWrongResourceInterface,
    ErrorRandRead,
    ErrorMemoryViewNotFound,
    ErrorMemoryViewNotCloser,
    ErrorSeekMethodNotFound,
    ErrorInvalidBool,
    ErrorFifoNotFound,
    ErrorFifoDatatypeInvalid,
    ErrorCap,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Error {
    pub id: u32,
}

impl Error {
    pub fn ok(&self) -> bool {
        self.id == 0
    }

    pub fn is_err(&self) -> bool {
        self.id != 0
    }

    pub fn is_errno(&self, err: Errno) -> bool {
        self.id == (err as u32)
    }
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ERROR_STRINGS[self.id as usize])
    }
}

pub trait ErrorTrait {
    fn error(&self) -> Error;
}

impl Errno {
    pub fn error(&self) -> Error {
        Error { id: (*self as u32) }
    }
}
