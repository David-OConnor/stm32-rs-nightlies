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
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when one or more Flash memory operation (programming / erase) has been completed successfully"]
    Error = 1,
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
            false => EOPR_A::NoError,
            true => EOPR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == EOPR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == EOPR_A::Error
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW_AW {
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
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
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when a Flash memory operation (program / erase) completes unsuccessfully"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
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
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when a double-word address to be programmed contains a value different from '0xFFFF FFFF' before programming, except if the data to write is '0x0000 0000'"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
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
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when an address to be erased/programmed belongs to a writeprotected part (by WRP, PCROP or RDP level 1) of the Flash memory"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
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
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when the data to program cannot be contained in the same 64-bit Flash memory row in case of standard programming, or if there is a change of page during fast programming"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
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
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when the size of the access is a byte or half-word during a program or a fast program sequence. Only double word programming is allowed (consequently: word access)"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
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
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when a write access to the Flash memory is performed by the code while PG or FSTPG have not been set previously. Set also by hardware when PROGERR, SIZERR, PGAERR, WRPERR, MISSERR or FASTERR is set due to a previous programming error. Set also when trying to perform bank erase when DBANK=0 (or DB1M = 0)"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGSERRW_AW::Clear)
    }
}
#[doc = "Field `MISERR` reader - Fast programming data miss error"]
pub type MISERR_R = crate::BitReader<MISERRR_A>;
#[doc = "Fast programming data miss error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISERRR_A {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: In fast programming mode, 32 double words must be sent to Flash successively, and the new data must be sent to the Flash logic control before the current data is fully programmed. MISSERR is set by hardware when the new data is not present in time"]
    Error = 1,
}
impl From<MISERRR_A> for bool {
    #[inline(always)]
    fn from(variant: MISERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl MISERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MISERRR_A {
        match self.bits {
            false => MISERRR_A::NoError,
            true => MISERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MISERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MISERRR_A::Error
    }
}
#[doc = "Fast programming data miss error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISERRW_AW {
    #[doc = "1: Cleared by writing 1"]
    Clear = 1,
}
impl From<MISERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: MISERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISERR` writer - Fast programming data miss error"]
pub type MISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, MISERRW_AW, O>;
impl<'a, const O: u8> MISERR_W<'a, O> {
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MISERRW_AW::Clear)
    }
}
#[doc = "Field `FASTERR` reader - Fast programming error"]
pub type FASTERR_R = crate::BitReader<FASTERRR_A>;
#[doc = "Fast programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRR_A {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when a fast programming sequence (activated by FSTPG) is interrupted due to an error (alignment, size, write protection or data miss). The corresponding status bit (PGAERR, SIZERR, WRPERR or MISSERR) is set at the same time"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FASTERRW_AW::Clear)
    }
}
#[doc = "Field `RDERR` reader - PCROP read error"]
pub type RDERR_R = crate::BitReader<RDERRR_A>;
#[doc = "PCROP read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRR_A {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when an address to be read through the D-bus belongs to a read protected area of the Flash (PCROP protection)"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
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
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Set by hardware when the options read may not be the one configured by the user. If option haven’t been properly loaded, OPTVERR is set again after each system reset"]
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
    #[doc = "1: Cleared by writing 1"]
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
    #[doc = "Cleared by writing 1"]
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
    #[doc = "0: Not busy"]
    NotBusy = 0,
    #[doc = "1: Busy"]
    Busy = 1,
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
            false => BSY_A::NotBusy,
            true => BSY_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `NotBusy`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSY_A::NotBusy
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSY_A::Busy
    }
}
#[doc = "Field `PEMPTY` reader - "]
pub type PEMPTY_R = crate::BitReader<PEMPTY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEMPTY_A {
    #[doc = "0: The bit value is toggling"]
    Toggling = 0,
    #[doc = "1: No effect"]
    NoEffect = 1,
}
impl From<PEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: PEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl PEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEMPTY_A {
        match self.bits {
            false => PEMPTY_A::Toggling,
            true => PEMPTY_A::NoEffect,
        }
    }
    #[doc = "Checks if the value of the field is `Toggling`"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        *self == PEMPTY_A::Toggling
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PEMPTY_A::NoEffect
    }
}
#[doc = "Field `PEMPTY` writer - "]
pub type PEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, PEMPTY_A, O>;
impl<'a, const O: u8> PEMPTY_W<'a, O> {
    #[doc = "The bit value is toggling"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut W {
        self.variant(PEMPTY_A::Toggling)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PEMPTY_A::NoEffect)
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
    pub fn miserr(&self) -> MISERR_R {
        MISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pempty(&self) -> PEMPTY_R {
        PEMPTY_R::new(((self.bits >> 17) & 1) != 0)
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
    pub fn miserr(&mut self) -> MISERR_W<8> {
        MISERR_W::new(self)
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
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pempty(&mut self) -> PEMPTY_W<17> {
        PEMPTY_W::new(self)
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
