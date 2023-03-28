#[doc = "Register `RSTC2R` reader"]
pub struct R(crate::R<RSTC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTC2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTC2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTC2R` writer"]
pub struct W(crate::W<RSTC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTC2R_SPEC>;
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
impl From<crate::W<RSTC2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTC2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRT` reader - SRT"]
pub type SRT_R = crate::BitReader<SRT_A>;
#[doc = "SRT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRT_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Force output to its inactive state"]
    SetInactive = 1,
}
impl From<SRT_A> for bool {
    #[inline(always)]
    fn from(variant: SRT_A) -> Self {
        variant as u8 != 0
    }
}
impl SRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRT_A {
        match self.bits {
            false => SRT_A::NoEffect,
            true => SRT_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SRT_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == SRT_A::SetInactive
    }
}
#[doc = "Field `SRT` writer - SRT"]
pub type SRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, SRT_A, O>;
impl<'a, const O: u8> SRT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SRT_A::NoEffect)
    }
    #[doc = "Force output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(SRT_A::SetInactive)
    }
}
#[doc = "Field `RESYNC` reader - RESYNC"]
pub type RESYNC_R = crate::BitReader<RESYNC_A>;
#[doc = "RESYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESYNC_A {
    #[doc = "0: Timer reset event coming solely from software or SYNC input event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer reset event coming solely from software or SYNC input event forces the output to its inactive state"]
    SetInactive = 1,
}
impl From<RESYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl RESYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESYNC_A {
        match self.bits {
            false => RESYNC_A::NoEffect,
            true => RESYNC_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RESYNC_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == RESYNC_A::SetInactive
    }
}
#[doc = "Field `RESYNC` writer - RESYNC"]
pub type RESYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, RESYNC_A, O>;
impl<'a, const O: u8> RESYNC_W<'a, O> {
    #[doc = "Timer reset event coming solely from software or SYNC input event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RESYNC_A::NoEffect)
    }
    #[doc = "Timer reset event coming solely from software or SYNC input event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(RESYNC_A::SetInactive)
    }
}
#[doc = "Field `PER` reader - PER"]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "PER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "0: Timer period event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer period event forces the output to its inactive state"]
    SetInactive = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::NoEffect,
            true => PER_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PER_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == PER_A::SetInactive
    }
}
#[doc = "Field `PER` writer - PER"]
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, PER_A, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    #[doc = "Timer period event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PER_A::NoEffect)
    }
    #[doc = "Timer period event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(PER_A::SetInactive)
    }
}
#[doc = "Field `CMP1` reader - CMP1"]
pub type CMP1_R = crate::BitReader<CMP1_A>;
#[doc = "CMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1_A {
    #[doc = "0: Timer compare event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer compare event forces the output to its inactive state"]
    SetInactive = 1,
}
impl From<CMP1_A> for bool {
    #[inline(always)]
    fn from(variant: CMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP1_A {
        match self.bits {
            false => CMP1_A::NoEffect,
            true => CMP1_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CMP1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == CMP1_A::SetInactive
    }
}
#[doc = "Field `CMP1` writer - CMP1"]
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, CMP1_A, O>;
impl<'a, const O: u8> CMP1_W<'a, O> {
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP1_A::NoEffect)
    }
    #[doc = "Timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(CMP1_A::SetInactive)
    }
}
#[doc = "Field `CMP2` reader - CMP2"]
pub use CMP1_R as CMP2_R;
#[doc = "Field `CMP3` reader - CMP3"]
pub use CMP1_R as CMP3_R;
#[doc = "Field `CMP4` reader - CMP4"]
pub use CMP1_R as CMP4_R;
#[doc = "Field `CMP2` writer - CMP2"]
pub use CMP1_W as CMP2_W;
#[doc = "Field `CMP3` writer - CMP3"]
pub use CMP1_W as CMP3_W;
#[doc = "Field `CMP4` writer - CMP4"]
pub use CMP1_W as CMP4_W;
#[doc = "Field `MSTPER` reader - MSTPER"]
pub type MSTPER_R = crate::BitReader<MSTPER_A>;
#[doc = "MSTPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPER_A {
    #[doc = "0: Master timer counter roll-over/reset has no effect"]
    NoEffect = 0,
    #[doc = "1: Master timer counter roll-over/reset forces the output to its inactive state"]
    SetInactive = 1,
}
impl From<MSTPER_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPER_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPER_A {
        match self.bits {
            false => MSTPER_A::NoEffect,
            true => MSTPER_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTPER_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == MSTPER_A::SetInactive
    }
}
#[doc = "Field `MSTPER` writer - MSTPER"]
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, MSTPER_A, O>;
impl<'a, const O: u8> MSTPER_W<'a, O> {
    #[doc = "Master timer counter roll-over/reset has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTPER_A::NoEffect)
    }
    #[doc = "Master timer counter roll-over/reset forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTPER_A::SetInactive)
    }
}
#[doc = "Field `MSTCMP1` reader - MSTCMP1"]
pub type MSTCMP1_R = crate::BitReader<MSTCMP1_A>;
#[doc = "MSTCMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1_A {
    #[doc = "0: Master timer compare event has no effect"]
    NoEffect = 0,
    #[doc = "1: Master timer compare event forces the output to its inactive state"]
    SetInactive = 1,
}
impl From<MSTCMP1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTCMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTCMP1_A {
        match self.bits {
            false => MSTCMP1_A::NoEffect,
            true => MSTCMP1_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == MSTCMP1_A::SetInactive
    }
}
#[doc = "Field `MSTCMP1` writer - MSTCMP1"]
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, MSTCMP1_A, O>;
impl<'a, const O: u8> MSTCMP1_W<'a, O> {
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP1_A::NoEffect)
    }
    #[doc = "Master timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTCMP1_A::SetInactive)
    }
}
#[doc = "Field `MSTCMP2` reader - MSTCMP2"]
pub use MSTCMP1_R as MSTCMP2_R;
#[doc = "Field `MSTCMP3` reader - MSTCMP3"]
pub use MSTCMP1_R as MSTCMP3_R;
#[doc = "Field `MSTCMP4` reader - MSTCMP4"]
pub use MSTCMP1_R as MSTCMP4_R;
#[doc = "Field `MSTCMP2` writer - MSTCMP2"]
pub use MSTCMP1_W as MSTCMP2_W;
#[doc = "Field `MSTCMP3` writer - MSTCMP3"]
pub use MSTCMP1_W as MSTCMP3_W;
#[doc = "Field `MSTCMP4` writer - MSTCMP4"]
pub use MSTCMP1_W as MSTCMP4_W;
#[doc = "Field `TIMEVNT1` reader - TIMEVNT1"]
pub type TIMEVNT1_R = crate::BitReader<TIMEVNT1_A>;
#[doc = "TIMEVNT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEVNT1_A {
    #[doc = "0: Timer event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer event forces the output to its inactive state"]
    SetInactive = 1,
}
impl From<TIMEVNT1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEVNT1_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEVNT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEVNT1_A {
        match self.bits {
            false => TIMEVNT1_A::NoEffect,
            true => TIMEVNT1_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIMEVNT1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == TIMEVNT1_A::SetInactive
    }
}
#[doc = "Field `TIMEVNT1` writer - TIMEVNT1"]
pub type TIMEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, TIMEVNT1_A, O>;
impl<'a, const O: u8> TIMEVNT1_W<'a, O> {
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT1_A::NoEffect)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT1_A::SetInactive)
    }
}
#[doc = "Field `TIMEVNT2` reader - TIMEVNT2"]
pub use TIMEVNT1_R as TIMEVNT2_R;
#[doc = "Field `TIMEVNT3` reader - TIMEVNT3"]
pub use TIMEVNT1_R as TIMEVNT3_R;
#[doc = "Field `TIMEVNT4` reader - TIMEVNT4"]
pub use TIMEVNT1_R as TIMEVNT4_R;
#[doc = "Field `TIMEVNT5` reader - TIMEVNT5"]
pub use TIMEVNT1_R as TIMEVNT5_R;
#[doc = "Field `TIMEVNT6` reader - TIMEVNT6"]
pub use TIMEVNT1_R as TIMEVNT6_R;
#[doc = "Field `TIMEVNT7` reader - TIMEVNT7"]
pub use TIMEVNT1_R as TIMEVNT7_R;
#[doc = "Field `TIMEVNT8` reader - TIMEVNT8"]
pub use TIMEVNT1_R as TIMEVNT8_R;
#[doc = "Field `TIMEVNT9` reader - TIMEVNT9"]
pub use TIMEVNT1_R as TIMEVNT9_R;
#[doc = "Field `TIMEVNT2` writer - TIMEVNT2"]
pub use TIMEVNT1_W as TIMEVNT2_W;
#[doc = "Field `TIMEVNT3` writer - TIMEVNT3"]
pub use TIMEVNT1_W as TIMEVNT3_W;
#[doc = "Field `TIMEVNT4` writer - TIMEVNT4"]
pub use TIMEVNT1_W as TIMEVNT4_W;
#[doc = "Field `TIMEVNT5` writer - TIMEVNT5"]
pub use TIMEVNT1_W as TIMEVNT5_W;
#[doc = "Field `TIMEVNT6` writer - TIMEVNT6"]
pub use TIMEVNT1_W as TIMEVNT6_W;
#[doc = "Field `TIMEVNT7` writer - TIMEVNT7"]
pub use TIMEVNT1_W as TIMEVNT7_W;
#[doc = "Field `TIMEVNT8` writer - TIMEVNT8"]
pub use TIMEVNT1_W as TIMEVNT8_W;
#[doc = "Field `TIMEVNT9` writer - TIMEVNT9"]
pub use TIMEVNT1_W as TIMEVNT9_W;
#[doc = "Field `EXTEVNT1` reader - EXTEVNT1"]
pub type EXTEVNT1_R = crate::BitReader<EXTEVNT1_A>;
#[doc = "EXTEVNT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEVNT1_A {
    #[doc = "0: External event has no effect"]
    NoEffect = 0,
    #[doc = "1: External event forces the output to its inactive state"]
    SetInactive = 1,
}
impl From<EXTEVNT1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEVNT1_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTEVNT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEVNT1_A {
        match self.bits {
            false => EXTEVNT1_A::NoEffect,
            true => EXTEVNT1_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXTEVNT1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == EXTEVNT1_A::SetInactive
    }
}
#[doc = "Field `EXTEVNT1` writer - EXTEVNT1"]
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, EXTEVNT1_A, O>;
impl<'a, const O: u8> EXTEVNT1_W<'a, O> {
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::NoEffect)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::SetInactive)
    }
}
#[doc = "Field `EXTEVNT2` reader - EXTEVNT2"]
pub use EXTEVNT1_R as EXTEVNT2_R;
#[doc = "Field `EXTEVNT3` reader - EXTEVNT3"]
pub use EXTEVNT1_R as EXTEVNT3_R;
#[doc = "Field `EXTEVNT4` reader - EXTEVNT4"]
pub use EXTEVNT1_R as EXTEVNT4_R;
#[doc = "Field `EXTEVNT5` reader - EXTEVNT5"]
pub use EXTEVNT1_R as EXTEVNT5_R;
#[doc = "Field `EXTEVNT6` reader - EXTEVNT6"]
pub use EXTEVNT1_R as EXTEVNT6_R;
#[doc = "Field `EXTEVNT7` reader - EXTEVNT7"]
pub use EXTEVNT1_R as EXTEVNT7_R;
#[doc = "Field `EXTEVNT8` reader - EXTEVNT8"]
pub use EXTEVNT1_R as EXTEVNT8_R;
#[doc = "Field `EXTEVNT9` reader - EXTEVNT9"]
pub use EXTEVNT1_R as EXTEVNT9_R;
#[doc = "Field `EXTEVNT10` reader - EXTEVNT10"]
pub use EXTEVNT1_R as EXTEVNT10_R;
#[doc = "Field `EXTEVNT2` writer - EXTEVNT2"]
pub use EXTEVNT1_W as EXTEVNT2_W;
#[doc = "Field `EXTEVNT3` writer - EXTEVNT3"]
pub use EXTEVNT1_W as EXTEVNT3_W;
#[doc = "Field `EXTEVNT4` writer - EXTEVNT4"]
pub use EXTEVNT1_W as EXTEVNT4_W;
#[doc = "Field `EXTEVNT5` writer - EXTEVNT5"]
pub use EXTEVNT1_W as EXTEVNT5_W;
#[doc = "Field `EXTEVNT6` writer - EXTEVNT6"]
pub use EXTEVNT1_W as EXTEVNT6_W;
#[doc = "Field `EXTEVNT7` writer - EXTEVNT7"]
pub use EXTEVNT1_W as EXTEVNT7_W;
#[doc = "Field `EXTEVNT8` writer - EXTEVNT8"]
pub use EXTEVNT1_W as EXTEVNT8_W;
#[doc = "Field `EXTEVNT9` writer - EXTEVNT9"]
pub use EXTEVNT1_W as EXTEVNT9_W;
#[doc = "Field `EXTEVNT10` writer - EXTEVNT10"]
pub use EXTEVNT1_W as EXTEVNT10_W;
#[doc = "Field `UPDATE` reader - UPDATE"]
pub type UPDATE_R = crate::BitReader<UPDATE_A>;
#[doc = "UPDATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDATE_A {
    #[doc = "0: Register update event has no effect"]
    NoEffect = 0,
    #[doc = "1: Register update event forces the output to its inactive state"]
    SetInactive = 1,
}
impl From<UPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATE_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATE_A {
        match self.bits {
            false => UPDATE_A::NoEffect,
            true => UPDATE_A::SetInactive,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDATE_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `SetInactive`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == UPDATE_A::SetInactive
    }
}
#[doc = "Field `UPDATE` writer - UPDATE"]
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTC2R_SPEC, UPDATE_A, O>;
impl<'a, const O: u8> UPDATE_W<'a, O> {
    #[doc = "Register update event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UPDATE_A::NoEffect)
    }
    #[doc = "Register update event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(UPDATE_A::SetInactive)
    }
}
impl R {
    #[doc = "Bit 0 - SRT"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RESYNC"]
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PER"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP1"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMP2"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP3"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMP4"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MSTPER"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMEVNT1"]
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIMEVNT2"]
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIMEVNT3"]
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMEVNT4"]
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMEVNT5"]
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMEVNT6"]
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMEVNT7"]
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TIMEVNT8"]
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIMEVNT9"]
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EXTEVNT1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EXTEVNT2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EXTEVNT3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EXTEVNT4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EXTEVNT5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EXTEVNT6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EXTEVNT7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXTEVNT8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EXTEVNT9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EXTEVNT10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UPDATE"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRT"]
    #[inline(always)]
    #[must_use]
    pub fn srt(&mut self) -> SRT_W<0> {
        SRT_W::new(self)
    }
    #[doc = "Bit 1 - RESYNC"]
    #[inline(always)]
    #[must_use]
    pub fn resync(&mut self) -> RESYNC_W<1> {
        RESYNC_W::new(self)
    }
    #[doc = "Bit 2 - PER"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<2> {
        PER_W::new(self)
    }
    #[doc = "Bit 3 - CMP1"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<3> {
        CMP1_W::new(self)
    }
    #[doc = "Bit 4 - CMP2"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<4> {
        CMP2_W::new(self)
    }
    #[doc = "Bit 5 - CMP3"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3(&mut self) -> CMP3_W<5> {
        CMP3_W::new(self)
    }
    #[doc = "Bit 6 - CMP4"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<6> {
        CMP4_W::new(self)
    }
    #[doc = "Bit 7 - MSTPER"]
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<7> {
        MSTPER_W::new(self)
    }
    #[doc = "Bit 8 - MSTCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<8> {
        MSTCMP1_W::new(self)
    }
    #[doc = "Bit 9 - MSTCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<9> {
        MSTCMP2_W::new(self)
    }
    #[doc = "Bit 10 - MSTCMP3"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<10> {
        MSTCMP3_W::new(self)
    }
    #[doc = "Bit 11 - MSTCMP4"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<11> {
        MSTCMP4_W::new(self)
    }
    #[doc = "Bit 12 - TIMEVNT1"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W<12> {
        TIMEVNT1_W::new(self)
    }
    #[doc = "Bit 13 - TIMEVNT2"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W<13> {
        TIMEVNT2_W::new(self)
    }
    #[doc = "Bit 14 - TIMEVNT3"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W<14> {
        TIMEVNT3_W::new(self)
    }
    #[doc = "Bit 15 - TIMEVNT4"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W<15> {
        TIMEVNT4_W::new(self)
    }
    #[doc = "Bit 16 - TIMEVNT5"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W<16> {
        TIMEVNT5_W::new(self)
    }
    #[doc = "Bit 17 - TIMEVNT6"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W<17> {
        TIMEVNT6_W::new(self)
    }
    #[doc = "Bit 18 - TIMEVNT7"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W<18> {
        TIMEVNT7_W::new(self)
    }
    #[doc = "Bit 19 - TIMEVNT8"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W<19> {
        TIMEVNT8_W::new(self)
    }
    #[doc = "Bit 20 - TIMEVNT9"]
    #[inline(always)]
    #[must_use]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W<20> {
        TIMEVNT9_W::new(self)
    }
    #[doc = "Bit 21 - EXTEVNT1"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<21> {
        EXTEVNT1_W::new(self)
    }
    #[doc = "Bit 22 - EXTEVNT2"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<22> {
        EXTEVNT2_W::new(self)
    }
    #[doc = "Bit 23 - EXTEVNT3"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<23> {
        EXTEVNT3_W::new(self)
    }
    #[doc = "Bit 24 - EXTEVNT4"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<24> {
        EXTEVNT4_W::new(self)
    }
    #[doc = "Bit 25 - EXTEVNT5"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<25> {
        EXTEVNT5_W::new(self)
    }
    #[doc = "Bit 26 - EXTEVNT6"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<26> {
        EXTEVNT6_W::new(self)
    }
    #[doc = "Bit 27 - EXTEVNT7"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<27> {
        EXTEVNT7_W::new(self)
    }
    #[doc = "Bit 28 - EXTEVNT8"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<28> {
        EXTEVNT8_W::new(self)
    }
    #[doc = "Bit 29 - EXTEVNT9"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<29> {
        EXTEVNT9_W::new(self)
    }
    #[doc = "Bit 30 - EXTEVNT10"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<30> {
        EXTEVNT10_W::new(self)
    }
    #[doc = "Bit 31 - UPDATE"]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<31> {
        UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Output2 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc2r](index.html) module"]
pub struct RSTC2R_SPEC;
impl crate::RegisterSpec for RSTC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstc2r::R](R) reader structure"]
impl crate::Readable for RSTC2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstc2r::W](W) writer structure"]
impl crate::Writable for RSTC2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTC2R to value 0"]
impl crate::Resettable for RSTC2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
