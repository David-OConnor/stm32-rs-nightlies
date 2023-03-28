#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader<EOPR_A>;
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPR_A {
    #[doc = "0: No EOP operation occurred"]
    NoEvent = 0,
    #[doc = "1: An EOP event occurred"]
    Event = 1,
}
impl From<EOPR_A> for bool {
    #[inline(always)]
    fn from(variant: EOPR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOPR_A {
        match self.bits {
            false => EOPR_A::NoEvent,
            true => EOPR_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOPR_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOPR_A::Event
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<EOPW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, EOPW_AW, O>;
impl<'a, const O: u8> EOP_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOPW_AW::Clear)
    }
}
#[doc = "Field `OPERR` reader - Operation error"]
pub type OPERR_R = crate::BitReader<OPERRR_A>;
#[doc = "Operation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRR_A {
    #[doc = "0: No memory opreation error happened"]
    NoError = 0,
    #[doc = "1: Memory operation error happened"]
    Error = 1,
}
impl From<OPERRR_A> for bool {
    #[inline(always)]
    fn from(variant: OPERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPERRR_A {
        match self.bits {
            false => OPERRR_A::NoError,
            true => OPERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPERRR_A::Error
    }
}
#[doc = "Operation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<OPERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OPERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` writer - Operation error"]
pub type OPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, OPERRW_AW, O>;
impl<'a, const O: u8> OPERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPERRW_AW::Clear)
    }
}
#[doc = "Field `PROGERR` reader - Programming error"]
pub type PROGERR_R = crate::BitReader<PROGERRR_A>;
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGERRR_A {
    #[doc = "0: No size programming error happened"]
    NoError = 0,
    #[doc = "1: Programming error happened"]
    Error = 1,
}
impl From<PROGERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PROGERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PROGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROGERRR_A {
        match self.bits {
            false => PROGERRR_A::NoError,
            true => PROGERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PROGERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PROGERRR_A::Error
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<PROGERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PROGERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROGERR` writer - Programming error"]
pub type PROGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, PROGERRW_AW, O>;
impl<'a, const O: u8> PROGERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PROGERRW_AW::Clear)
    }
}
#[doc = "Field `WRPERR` reader - Write protected error"]
pub type WRPERR_R = crate::BitReader<WRPERRR_A>;
#[doc = "Write protected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRR_A {
    #[doc = "0: No write protection error happened"]
    NoError = 0,
    #[doc = "1: Write protection error happened"]
    Error = 1,
}
impl From<WRPERRR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl WRPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPERRR_A {
        match self.bits {
            false => WRPERRR_A::NoError,
            true => WRPERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPERRR_A::Error
    }
}
#[doc = "Write protected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<WRPERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: WRPERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` writer - Write protected error"]
pub type WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, WRPERRW_AW, O>;
impl<'a, const O: u8> WRPERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WRPERRW_AW::Clear)
    }
}
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PGAERR_R = crate::BitReader<PGAERRR_A>;
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRR_A {
    #[doc = "0: No programming alignment error happened"]
    NoError = 0,
    #[doc = "1: Programming alignment error happened"]
    Error = 1,
}
impl From<PGAERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PGAERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PGAERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAERRR_A {
        match self.bits {
            false => PGAERRR_A::NoError,
            true => PGAERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGAERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGAERRR_A::Error
    }
}
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<PGAERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PGAERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PGAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, PGAERRW_AW, O>;
impl<'a, const O: u8> PGAERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGAERRW_AW::Clear)
    }
}
#[doc = "Field `SIZERR` reader - Size error"]
pub type SIZERR_R = crate::BitReader<SIZERRR_A>;
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRR_A {
    #[doc = "0: No size error happened"]
    NoError = 0,
    #[doc = "1: Size error happened"]
    Error = 1,
}
impl From<SIZERRR_A> for bool {
    #[inline(always)]
    fn from(variant: SIZERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl SIZERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIZERRR_A {
        match self.bits {
            false => SIZERRR_A::NoError,
            true => SIZERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SIZERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SIZERRR_A::Error
    }
}
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<SIZERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: SIZERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZERR` writer - Size error"]
pub type SIZERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, SIZERRW_AW, O>;
impl<'a, const O: u8> SIZERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SIZERRW_AW::Clear)
    }
}
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub type PGSERR_R = crate::BitReader<PGSERRR_A>;
#[doc = "Programming sequence error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRR_A {
    #[doc = "0: No fast programming sequence error happened"]
    NoError = 0,
    #[doc = "1: Fast programming sequence error happened"]
    Error = 1,
}
impl From<PGSERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PGSERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PGSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGSERRR_A {
        match self.bits {
            false => PGSERRR_A::NoError,
            true => PGSERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGSERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGSERRR_A::Error
    }
}
#[doc = "Programming sequence error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<PGSERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PGSERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub type PGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, PGSERRW_AW, O>;
impl<'a, const O: u8> PGSERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGSERRW_AW::Clear)
    }
}
#[doc = "Field `MISSERR` reader - Fast programming data miss error"]
pub type MISSERR_R = crate::BitReader<MISSERRR_A>;
#[doc = "Fast programming data miss error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISSERRR_A {
    #[doc = "0: No fast programming data miss error happened"]
    NoError = 0,
    #[doc = "1: Fast programming data miss error happened"]
    Error = 1,
}
impl From<MISSERRR_A> for bool {
    #[inline(always)]
    fn from(variant: MISSERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl MISSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MISSERRR_A {
        match self.bits {
            false => MISSERRR_A::NoError,
            true => MISSERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MISSERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MISSERRR_A::Error
    }
}
#[doc = "Fast programming data miss error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISSERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<MISSERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: MISSERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISSERR` writer - Fast programming data miss error"]
pub type MISSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, MISSERRW_AW, O>;
impl<'a, const O: u8> MISSERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MISSERRW_AW::Clear)
    }
}
#[doc = "Field `FASTERR` reader - Fast programming error"]
pub type FASTERR_R = crate::BitReader<FASTERRR_A>;
#[doc = "Fast programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRR_A {
    #[doc = "0: No fast programming error happened"]
    NoError = 0,
    #[doc = "1: Fast programming error happened"]
    Error = 1,
}
impl From<FASTERRR_A> for bool {
    #[inline(always)]
    fn from(variant: FASTERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl FASTERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FASTERRR_A {
        match self.bits {
            false => FASTERRR_A::NoError,
            true => FASTERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FASTERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FASTERRR_A::Error
    }
}
#[doc = "Fast programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<FASTERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: FASTERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTERR` writer - Fast programming error"]
pub type FASTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, FASTERRW_AW, O>;
impl<'a, const O: u8> FASTERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FASTERRW_AW::Clear)
    }
}
#[doc = "Field `OPTNV` reader - User Option OPTIVAL indication"]
pub type OPTNV_R = crate::BitReader<OPTNV_A>;
#[doc = "User Option OPTIVAL indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTNV_A {
    #[doc = "0: The OBL user option OPTVAL indicates \"valid\""]
    Valid = 0,
    #[doc = "1: The OBL user option OPTVAL indicates \"invalid\""]
    Invalid = 1,
}
impl From<OPTNV_A> for bool {
    #[inline(always)]
    fn from(variant: OPTNV_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTNV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPTNV_A {
        match self.bits {
            false => OPTNV_A::Valid,
            true => OPTNV_A::Invalid,
        }
    }
    #[doc = "Checks if the value of the field is `Valid`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == OPTNV_A::Valid
    }
    #[doc = "Checks if the value of the field is `Invalid`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == OPTNV_A::Invalid
    }
}
#[doc = "Field `RDERR` reader - PCROP read error"]
pub type RDERR_R = crate::BitReader<RDERRR_A>;
#[doc = "PCROP read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRR_A {
    #[doc = "0: No read-only error happened"]
    NoError = 0,
    #[doc = "1: Read-only error happened"]
    Error = 1,
}
impl From<RDERRR_A> for bool {
    #[inline(always)]
    fn from(variant: RDERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl RDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDERRR_A {
        match self.bits {
            false => RDERRR_A::NoError,
            true => RDERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERRR_A::Error
    }
}
#[doc = "PCROP read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<RDERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: RDERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` writer - PCROP read error"]
pub type RDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, RDERRW_AW, O>;
impl<'a, const O: u8> RDERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RDERRW_AW::Clear)
    }
}
#[doc = "Field `OPTVERR` reader - Option validity error"]
pub type OPTVERR_R = crate::BitReader<OPTVERRR_A>;
#[doc = "Option validity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRR_A {
    #[doc = "0: No error in option and engineering bits"]
    NoError = 0,
    #[doc = "1: Error in option and engineering bits"]
    Error = 1,
}
impl From<OPTVERRR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTVERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPTVERRR_A {
        match self.bits {
            false => OPTVERRR_A::NoError,
            true => OPTVERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPTVERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPTVERRR_A::Error
    }
}
#[doc = "Option validity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<OPTVERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTVERR` writer - Option validity error"]
pub type OPTVERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, OPTVERRW_AW, O>;
impl<'a, const O: u8> OPTVERR_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPTVERRW_AW::Clear)
    }
}
#[doc = "Field `BSY` reader - Busy"]
pub type BSY_R = crate::BitReader<BSY_A>;
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSY_A {
    #[doc = "0: No write/erase operation is in progress"]
    Inactive = 0,
    #[doc = "1: No write/erase operation is in progress"]
    Active = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::Inactive,
            true => BSY_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `Inactive`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSY_A::Inactive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSY_A::Active
    }
}
#[doc = "Field `CFGBSY` reader - Programming or erase configuration busy"]
pub type CFGBSY_R = crate::BitReader<CFGBSY_A>;
#[doc = "Programming or erase configuration busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFGBSY_A {
    #[doc = "0: PG, PNB, PER, MER bits available for writing"]
    Free = 0,
    #[doc = "1: PG, PNB, PER, MER bits not available for writing (operation ongoing)"]
    Busy = 1,
}
impl From<CFGBSY_A> for bool {
    #[inline(always)]
    fn from(variant: CFGBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl CFGBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGBSY_A {
        match self.bits {
            false => CFGBSY_A::Free,
            true => CFGBSY_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `Free`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == CFGBSY_A::Free
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == CFGBSY_A::Busy
    }
}
#[doc = "Field `PESD` reader - Programming / erase operation suspended"]
pub type PESD_R = crate::BitReader<PESD_A>;
#[doc = "Programming / erase operation suspended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PESD_A {
    #[doc = "0: Flash program and erase operations granted"]
    Granted = 0,
    #[doc = "1: Any new Flash program and erase operation is suspended until this bit is cleared. This bit is set when the PES bit in FLASH_ACR is set"]
    Suspended = 1,
}
impl From<PESD_A> for bool {
    #[inline(always)]
    fn from(variant: PESD_A) -> Self {
        variant as u8 != 0
    }
}
impl PESD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESD_A {
        match self.bits {
            false => PESD_A::Granted,
            true => PESD_A::Suspended,
        }
    }
    #[doc = "Checks if the value of the field is `Granted`"]
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        *self == PESD_A::Granted
    }
    #[doc = "Checks if the value of the field is `Suspended`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == PESD_A::Suspended
    }
}
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn misserr(&self) -> MISSERR_R {
        MISSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - User Option OPTIVAL indication"]
    #[inline(always)]
    pub fn optnv(&self) -> OPTNV_R {
        OPTNV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Programming or erase configuration busy"]
    #[inline(always)]
    pub fn cfgbsy(&self) -> CFGBSY_R {
        CFGBSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Programming / erase operation suspended"]
    #[inline(always)]
    pub fn pesd(&self) -> PESD_R {
        PESD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<0> {
        EOP_W::new(self)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<1> {
        OPERR_W::new(self)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<3> {
        PROGERR_W::new(self)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<4> {
        WRPERR_W::new(self)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<5> {
        PGAERR_W::new(self)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SIZERR_W<6> {
        SIZERR_W::new(self)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<7> {
        PGSERR_W::new(self)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    #[must_use]
    pub fn misserr(&mut self) -> MISSERR_W<8> {
        MISSERR_W::new(self)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    #[must_use]
    pub fn fasterr(&mut self) -> FASTERR_W<9> {
        FASTERR_W::new(self)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    #[must_use]
    pub fn rderr(&mut self) -> RDERR_W<14> {
        RDERR_W::new(self)
    }
    #[doc = "Bit 15 - Option validity error"]
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OPTVERR_W<15> {
        OPTVERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
