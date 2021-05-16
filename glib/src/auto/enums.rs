// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::error::ErrorDomain;
use crate::translate::*;
use crate::Quark;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GChecksumType")]
pub enum ChecksumType {
    #[doc(alias = "G_CHECKSUM_MD5")]
    Md5,
    #[doc(alias = "G_CHECKSUM_SHA1")]
    Sha1,
    #[doc(alias = "G_CHECKSUM_SHA256")]
    Sha256,
    #[doc(alias = "G_CHECKSUM_SHA512")]
    Sha512,
    #[cfg(any(feature = "v2_52", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_52")))]
    #[doc(alias = "G_CHECKSUM_SHA384")]
    Sha384,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ChecksumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ChecksumType::{}",
            match *self {
                Self::Md5 => "Md5",
                Self::Sha1 => "Sha1",
                Self::Sha256 => "Sha256",
                Self::Sha512 => "Sha512",
                #[cfg(any(feature = "v2_52", feature = "dox"))]
                Self::Sha384 => "Sha384",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ChecksumType {
    type GlibType = ffi::GChecksumType;

    fn into_glib(self) -> ffi::GChecksumType {
        match self {
            Self::Md5 => ffi::G_CHECKSUM_MD5,
            Self::Sha1 => ffi::G_CHECKSUM_SHA1,
            Self::Sha256 => ffi::G_CHECKSUM_SHA256,
            Self::Sha512 => ffi::G_CHECKSUM_SHA512,
            #[cfg(any(feature = "v2_52", feature = "dox"))]
            Self::Sha384 => ffi::G_CHECKSUM_SHA384,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GChecksumType> for ChecksumType {
    unsafe fn from_glib(value: ffi::GChecksumType) -> Self {
        match value {
            0 => Self::Md5,
            1 => Self::Sha1,
            2 => Self::Sha256,
            3 => Self::Sha512,
            #[cfg(any(feature = "v2_52", feature = "dox"))]
            4 => Self::Sha384,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GDateMonth")]
pub enum DateMonth {
    #[doc(alias = "G_DATE_BAD_MONTH")]
    BadMonth,
    #[doc(alias = "G_DATE_JANUARY")]
    January,
    #[doc(alias = "G_DATE_FEBRUARY")]
    February,
    #[doc(alias = "G_DATE_MARCH")]
    March,
    #[doc(alias = "G_DATE_APRIL")]
    April,
    #[doc(alias = "G_DATE_MAY")]
    May,
    #[doc(alias = "G_DATE_JUNE")]
    June,
    #[doc(alias = "G_DATE_JULY")]
    July,
    #[doc(alias = "G_DATE_AUGUST")]
    August,
    #[doc(alias = "G_DATE_SEPTEMBER")]
    September,
    #[doc(alias = "G_DATE_OCTOBER")]
    October,
    #[doc(alias = "G_DATE_NOVEMBER")]
    November,
    #[doc(alias = "G_DATE_DECEMBER")]
    December,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DateMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DateMonth::{}",
            match *self {
                Self::BadMonth => "BadMonth",
                Self::January => "January",
                Self::February => "February",
                Self::March => "March",
                Self::April => "April",
                Self::May => "May",
                Self::June => "June",
                Self::July => "July",
                Self::August => "August",
                Self::September => "September",
                Self::October => "October",
                Self::November => "November",
                Self::December => "December",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for DateMonth {
    type GlibType = ffi::GDateMonth;

    fn into_glib(self) -> ffi::GDateMonth {
        match self {
            Self::BadMonth => ffi::G_DATE_BAD_MONTH,
            Self::January => ffi::G_DATE_JANUARY,
            Self::February => ffi::G_DATE_FEBRUARY,
            Self::March => ffi::G_DATE_MARCH,
            Self::April => ffi::G_DATE_APRIL,
            Self::May => ffi::G_DATE_MAY,
            Self::June => ffi::G_DATE_JUNE,
            Self::July => ffi::G_DATE_JULY,
            Self::August => ffi::G_DATE_AUGUST,
            Self::September => ffi::G_DATE_SEPTEMBER,
            Self::October => ffi::G_DATE_OCTOBER,
            Self::November => ffi::G_DATE_NOVEMBER,
            Self::December => ffi::G_DATE_DECEMBER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GDateMonth> for DateMonth {
    unsafe fn from_glib(value: ffi::GDateMonth) -> Self {
        match value {
            0 => Self::BadMonth,
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            12 => Self::December,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GDateWeekday")]
pub enum DateWeekday {
    #[doc(alias = "G_DATE_BAD_WEEKDAY")]
    BadWeekday,
    #[doc(alias = "G_DATE_MONDAY")]
    Monday,
    #[doc(alias = "G_DATE_TUESDAY")]
    Tuesday,
    #[doc(alias = "G_DATE_WEDNESDAY")]
    Wednesday,
    #[doc(alias = "G_DATE_THURSDAY")]
    Thursday,
    #[doc(alias = "G_DATE_FRIDAY")]
    Friday,
    #[doc(alias = "G_DATE_SATURDAY")]
    Saturday,
    #[doc(alias = "G_DATE_SUNDAY")]
    Sunday,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DateWeekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DateWeekday::{}",
            match *self {
                Self::BadWeekday => "BadWeekday",
                Self::Monday => "Monday",
                Self::Tuesday => "Tuesday",
                Self::Wednesday => "Wednesday",
                Self::Thursday => "Thursday",
                Self::Friday => "Friday",
                Self::Saturday => "Saturday",
                Self::Sunday => "Sunday",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for DateWeekday {
    type GlibType = ffi::GDateWeekday;

    fn into_glib(self) -> ffi::GDateWeekday {
        match self {
            Self::BadWeekday => ffi::G_DATE_BAD_WEEKDAY,
            Self::Monday => ffi::G_DATE_MONDAY,
            Self::Tuesday => ffi::G_DATE_TUESDAY,
            Self::Wednesday => ffi::G_DATE_WEDNESDAY,
            Self::Thursday => ffi::G_DATE_THURSDAY,
            Self::Friday => ffi::G_DATE_FRIDAY,
            Self::Saturday => ffi::G_DATE_SATURDAY,
            Self::Sunday => ffi::G_DATE_SUNDAY,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GDateWeekday> for DateWeekday {
    unsafe fn from_glib(value: ffi::GDateWeekday) -> Self {
        match value {
            0 => Self::BadWeekday,
            1 => Self::Monday,
            2 => Self::Tuesday,
            3 => Self::Wednesday,
            4 => Self::Thursday,
            5 => Self::Friday,
            6 => Self::Saturday,
            7 => Self::Sunday,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GKeyFileError")]
pub enum KeyFileError {
    #[doc(alias = "G_KEY_FILE_ERROR_UNKNOWN_ENCODING")]
    UnknownEncoding,
    #[doc(alias = "G_KEY_FILE_ERROR_PARSE")]
    Parse,
    #[doc(alias = "G_KEY_FILE_ERROR_NOT_FOUND")]
    NotFound,
    #[doc(alias = "G_KEY_FILE_ERROR_KEY_NOT_FOUND")]
    KeyNotFound,
    #[doc(alias = "G_KEY_FILE_ERROR_GROUP_NOT_FOUND")]
    GroupNotFound,
    #[doc(alias = "G_KEY_FILE_ERROR_INVALID_VALUE")]
    InvalidValue,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for KeyFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "KeyFileError::{}",
            match *self {
                Self::UnknownEncoding => "UnknownEncoding",
                Self::Parse => "Parse",
                Self::NotFound => "NotFound",
                Self::KeyNotFound => "KeyNotFound",
                Self::GroupNotFound => "GroupNotFound",
                Self::InvalidValue => "InvalidValue",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for KeyFileError {
    type GlibType = ffi::GKeyFileError;

    fn into_glib(self) -> ffi::GKeyFileError {
        match self {
            Self::UnknownEncoding => ffi::G_KEY_FILE_ERROR_UNKNOWN_ENCODING,
            Self::Parse => ffi::G_KEY_FILE_ERROR_PARSE,
            Self::NotFound => ffi::G_KEY_FILE_ERROR_NOT_FOUND,
            Self::KeyNotFound => ffi::G_KEY_FILE_ERROR_KEY_NOT_FOUND,
            Self::GroupNotFound => ffi::G_KEY_FILE_ERROR_GROUP_NOT_FOUND,
            Self::InvalidValue => ffi::G_KEY_FILE_ERROR_INVALID_VALUE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileError> for KeyFileError {
    unsafe fn from_glib(value: ffi::GKeyFileError) -> Self {
        match value {
            0 => Self::UnknownEncoding,
            1 => Self::Parse,
            2 => Self::NotFound,
            3 => Self::KeyNotFound,
            4 => Self::GroupNotFound,
            5 => Self::InvalidValue,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for KeyFileError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::g_key_file_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::UnknownEncoding),
            1 => Some(Self::Parse),
            2 => Some(Self::NotFound),
            3 => Some(Self::KeyNotFound),
            4 => Some(Self::GroupNotFound),
            5 => Some(Self::InvalidValue),
            value => Some(Self::__Unknown(value)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GOptionArg")]
pub enum OptionArg {
    #[doc(alias = "G_OPTION_ARG_NONE")]
    None,
    #[doc(alias = "G_OPTION_ARG_STRING")]
    String,
    #[doc(alias = "G_OPTION_ARG_INT")]
    Int,
    #[doc(alias = "G_OPTION_ARG_CALLBACK")]
    Callback,
    #[doc(alias = "G_OPTION_ARG_FILENAME")]
    Filename,
    #[doc(alias = "G_OPTION_ARG_STRING_ARRAY")]
    StringArray,
    #[doc(alias = "G_OPTION_ARG_FILENAME_ARRAY")]
    FilenameArray,
    #[doc(alias = "G_OPTION_ARG_DOUBLE")]
    Double,
    #[doc(alias = "G_OPTION_ARG_INT64")]
    Int64,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for OptionArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OptionArg::{}",
            match *self {
                Self::None => "None",
                Self::String => "String",
                Self::Int => "Int",
                Self::Callback => "Callback",
                Self::Filename => "Filename",
                Self::StringArray => "StringArray",
                Self::FilenameArray => "FilenameArray",
                Self::Double => "Double",
                Self::Int64 => "Int64",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for OptionArg {
    type GlibType = ffi::GOptionArg;

    fn into_glib(self) -> ffi::GOptionArg {
        match self {
            Self::None => ffi::G_OPTION_ARG_NONE,
            Self::String => ffi::G_OPTION_ARG_STRING,
            Self::Int => ffi::G_OPTION_ARG_INT,
            Self::Callback => ffi::G_OPTION_ARG_CALLBACK,
            Self::Filename => ffi::G_OPTION_ARG_FILENAME,
            Self::StringArray => ffi::G_OPTION_ARG_STRING_ARRAY,
            Self::FilenameArray => ffi::G_OPTION_ARG_FILENAME_ARRAY,
            Self::Double => ffi::G_OPTION_ARG_DOUBLE,
            Self::Int64 => ffi::G_OPTION_ARG_INT64,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GOptionArg> for OptionArg {
    unsafe fn from_glib(value: ffi::GOptionArg) -> Self {
        match value {
            0 => Self::None,
            1 => Self::String,
            2 => Self::Int,
            3 => Self::Callback,
            4 => Self::Filename,
            5 => Self::StringArray,
            6 => Self::FilenameArray,
            7 => Self::Double,
            8 => Self::Int64,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GSeekType")]
pub enum SeekType {
    #[doc(alias = "G_SEEK_CUR")]
    Cur,
    #[doc(alias = "G_SEEK_SET")]
    Set,
    #[doc(alias = "G_SEEK_END")]
    End,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SeekType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SeekType::{}",
            match *self {
                Self::Cur => "Cur",
                Self::Set => "Set",
                Self::End => "End",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SeekType {
    type GlibType = ffi::GSeekType;

    fn into_glib(self) -> ffi::GSeekType {
        match self {
            Self::Cur => ffi::G_SEEK_CUR,
            Self::Set => ffi::G_SEEK_SET,
            Self::End => ffi::G_SEEK_END,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSeekType> for SeekType {
    unsafe fn from_glib(value: ffi::GSeekType) -> Self {
        match value {
            0 => Self::Cur,
            1 => Self::Set,
            2 => Self::End,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GTimeType")]
pub enum TimeType {
    #[doc(alias = "G_TIME_TYPE_STANDARD")]
    Standard,
    #[doc(alias = "G_TIME_TYPE_DAYLIGHT")]
    Daylight,
    #[doc(alias = "G_TIME_TYPE_UNIVERSAL")]
    Universal,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TimeType::{}",
            match *self {
                Self::Standard => "Standard",
                Self::Daylight => "Daylight",
                Self::Universal => "Universal",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for TimeType {
    type GlibType = ffi::GTimeType;

    fn into_glib(self) -> ffi::GTimeType {
        match self {
            Self::Standard => ffi::G_TIME_TYPE_STANDARD,
            Self::Daylight => ffi::G_TIME_TYPE_DAYLIGHT,
            Self::Universal => ffi::G_TIME_TYPE_UNIVERSAL,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GTimeType> for TimeType {
    unsafe fn from_glib(value: ffi::GTimeType) -> Self {
        match value {
            0 => Self::Standard,
            1 => Self::Daylight,
            2 => Self::Universal,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GUriError")]
pub enum UriError {
    #[doc(alias = "G_URI_ERROR_FAILED")]
    Failed,
    #[doc(alias = "G_URI_ERROR_BAD_SCHEME")]
    BadScheme,
    #[doc(alias = "G_URI_ERROR_BAD_USER")]
    BadUser,
    #[doc(alias = "G_URI_ERROR_BAD_PASSWORD")]
    BadPassword,
    #[doc(alias = "G_URI_ERROR_BAD_AUTH_PARAMS")]
    BadAuthParams,
    #[doc(alias = "G_URI_ERROR_BAD_HOST")]
    BadHost,
    #[doc(alias = "G_URI_ERROR_BAD_PORT")]
    BadPort,
    #[doc(alias = "G_URI_ERROR_BAD_PATH")]
    BadPath,
    #[doc(alias = "G_URI_ERROR_BAD_QUERY")]
    BadQuery,
    #[doc(alias = "G_URI_ERROR_BAD_FRAGMENT")]
    BadFragment,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
impl fmt::Display for UriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UriError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::BadScheme => "BadScheme",
                Self::BadUser => "BadUser",
                Self::BadPassword => "BadPassword",
                Self::BadAuthParams => "BadAuthParams",
                Self::BadHost => "BadHost",
                Self::BadPort => "BadPort",
                Self::BadPath => "BadPath",
                Self::BadQuery => "BadQuery",
                Self::BadFragment => "BadFragment",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl IntoGlib for UriError {
    type GlibType = ffi::GUriError;

    fn into_glib(self) -> ffi::GUriError {
        match self {
            Self::Failed => ffi::G_URI_ERROR_FAILED,
            Self::BadScheme => ffi::G_URI_ERROR_BAD_SCHEME,
            Self::BadUser => ffi::G_URI_ERROR_BAD_USER,
            Self::BadPassword => ffi::G_URI_ERROR_BAD_PASSWORD,
            Self::BadAuthParams => ffi::G_URI_ERROR_BAD_AUTH_PARAMS,
            Self::BadHost => ffi::G_URI_ERROR_BAD_HOST,
            Self::BadPort => ffi::G_URI_ERROR_BAD_PORT,
            Self::BadPath => ffi::G_URI_ERROR_BAD_PATH,
            Self::BadQuery => ffi::G_URI_ERROR_BAD_QUERY,
            Self::BadFragment => ffi::G_URI_ERROR_BAD_FRAGMENT,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GUriError> for UriError {
    unsafe fn from_glib(value: ffi::GUriError) -> Self {
        match value {
            0 => Self::Failed,
            1 => Self::BadScheme,
            2 => Self::BadUser,
            3 => Self::BadPassword,
            4 => Self::BadAuthParams,
            5 => Self::BadHost,
            6 => Self::BadPort,
            7 => Self::BadPath,
            8 => Self::BadQuery,
            9 => Self::BadFragment,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
impl ErrorDomain for UriError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::g_uri_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Failed),
            1 => Some(Self::BadScheme),
            2 => Some(Self::BadUser),
            3 => Some(Self::BadPassword),
            4 => Some(Self::BadAuthParams),
            5 => Some(Self::BadHost),
            6 => Some(Self::BadPort),
            7 => Some(Self::BadPath),
            8 => Some(Self::BadQuery),
            9 => Some(Self::BadFragment),
            _ => Some(Self::Failed),
        }
    }
}