#[doc = "Register `RSTBR` reader"]
pub struct R(crate::R<RSTBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTBR` writer"]
pub struct W(crate::W<RSTBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTBR_SPEC>;
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
impl From<crate::W<RSTBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDT` reader - Timer A Update reset"]
pub type UPDT_R = crate::BitReader<UPDT_A>;
#[doc = "Timer A Update reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDT_A {
    #[doc = "0: Update event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X counter is reset upon update event"]
    ResetCounter = 1,
}
impl From<UPDT_A> for bool {
    #[inline(always)]
    fn from(variant: UPDT_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDT_A {
        match self.bits {
            false => UPDT_A::NoEffect,
            true => UPDT_A::ResetCounter,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDT_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `ResetCounter`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == UPDT_A::ResetCounter
    }
}
#[doc = "Field `UPDT` writer - Timer A Update reset"]
pub type UPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTBR_SPEC, UPDT_A, O>;
impl<'a, const O: u8> UPDT_W<'a, O> {
    #[doc = "Update event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UPDT_A::NoEffect)
    }
    #[doc = "Timer X counter is reset upon update event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(UPDT_A::ResetCounter)
    }
}
#[doc = "Field `CMP2` reader - Timer A compare 2 reset"]
pub type CMP2_R = crate::BitReader<CMP2_A>;
#[doc = "Timer A compare 2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP2_A {
    #[doc = "0: Timer X compare Z event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X counter is reset upon timer X compare Z event"]
    ResetCounter = 1,
}
impl From<CMP2_A> for bool {
    #[inline(always)]
    fn from(variant: CMP2_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP2_A {
        match self.bits {
            false => CMP2_A::NoEffect,
            true => CMP2_A::ResetCounter,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CMP2_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `ResetCounter`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == CMP2_A::ResetCounter
    }
}
#[doc = "Field `CMP2` writer - Timer A compare 2 reset"]
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTBR_SPEC, CMP2_A, O>;
impl<'a, const O: u8> CMP2_W<'a, O> {
    #[doc = "Timer X compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP2_A::NoEffect)
    }
    #[doc = "Timer X counter is reset upon timer X compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(CMP2_A::ResetCounter)
    }
}
#[doc = "Field `CMP4` reader - Timer A compare 4 reset"]
pub use CMP2_R as CMP4_R;
#[doc = "Field `CMP4` writer - Timer A compare 4 reset"]
pub use CMP2_W as CMP4_W;
#[doc = "Field `MSTPER` reader - Master timer Period"]
pub type MSTPER_R = crate::BitReader<MSTPER_A>;
#[doc = "Master timer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPER_A {
    #[doc = "0: Master timer period event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X counter is reset upon master timer period event"]
    ResetCounter = 1,
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
            true => MSTPER_A::ResetCounter,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTPER_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `ResetCounter`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == MSTPER_A::ResetCounter
    }
}
#[doc = "Field `MSTPER` writer - Master timer Period"]
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTBR_SPEC, MSTPER_A, O>;
impl<'a, const O: u8> MSTPER_W<'a, O> {
    #[doc = "Master timer period event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTPER_A::NoEffect)
    }
    #[doc = "Timer X counter is reset upon master timer period event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTPER_A::ResetCounter)
    }
}
#[doc = "Field `MSTCMP1` reader - Master compare 1"]
pub type MSTCMP1_R = crate::BitReader<MSTCMP1_A>;
#[doc = "Master compare 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1_A {
    #[doc = "0: Master timer compare Z event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X counter is reset upon master timer compare Z event"]
    ResetCounter = 1,
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
            true => MSTCMP1_A::ResetCounter,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `ResetCounter`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == MSTCMP1_A::ResetCounter
    }
}
#[doc = "Field `MSTCMP1` writer - Master compare 1"]
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTBR_SPEC, MSTCMP1_A, O>;
impl<'a, const O: u8> MSTCMP1_W<'a, O> {
    #[doc = "Master timer compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP1_A::NoEffect)
    }
    #[doc = "Timer X counter is reset upon master timer compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTCMP1_A::ResetCounter)
    }
}
#[doc = "Field `MSTCMP2` reader - Master compare 2"]
pub use MSTCMP1_R as MSTCMP2_R;
#[doc = "Field `MSTCMP3` reader - Master compare 3"]
pub use MSTCMP1_R as MSTCMP3_R;
#[doc = "Field `MSTCMP4` reader - Master compare 4"]
pub use MSTCMP1_R as MSTCMP4_R;
#[doc = "Field `MSTCMP2` writer - Master compare 2"]
pub use MSTCMP1_W as MSTCMP2_W;
#[doc = "Field `MSTCMP3` writer - Master compare 3"]
pub use MSTCMP1_W as MSTCMP3_W;
#[doc = "Field `MSTCMP4` writer - Master compare 4"]
pub use MSTCMP1_W as MSTCMP4_W;
#[doc = "Field `EXTEVNT1` reader - External Event 1"]
pub type EXTEVNT1_R = crate::BitReader<EXTEVNT1_A>;
#[doc = "External Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEVNT1_A {
    #[doc = "0: External event Z has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X counter is reset upon external event Z"]
    ResetCounter = 1,
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
            true => EXTEVNT1_A::ResetCounter,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXTEVNT1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `ResetCounter`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == EXTEVNT1_A::ResetCounter
    }
}
#[doc = "Field `EXTEVNT1` writer - External Event 1"]
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTBR_SPEC, EXTEVNT1_A, O>;
impl<'a, const O: u8> EXTEVNT1_W<'a, O> {
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::NoEffect)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::ResetCounter)
    }
}
#[doc = "Field `EXTEVNT2` reader - External Event 2"]
pub use EXTEVNT1_R as EXTEVNT2_R;
#[doc = "Field `EXTEVNT3` reader - External Event 3"]
pub use EXTEVNT1_R as EXTEVNT3_R;
#[doc = "Field `EXTEVNT4` reader - External Event 4"]
pub use EXTEVNT1_R as EXTEVNT4_R;
#[doc = "Field `EXTEVNT5` reader - External Event 5"]
pub use EXTEVNT1_R as EXTEVNT5_R;
#[doc = "Field `EXTEVNT6` reader - External Event 6"]
pub use EXTEVNT1_R as EXTEVNT6_R;
#[doc = "Field `EXTEVNT7` reader - External Event 7"]
pub use EXTEVNT1_R as EXTEVNT7_R;
#[doc = "Field `EXTEVNT8` reader - External Event 8"]
pub use EXTEVNT1_R as EXTEVNT8_R;
#[doc = "Field `EXTEVNT9` reader - External Event 9"]
pub use EXTEVNT1_R as EXTEVNT9_R;
#[doc = "Field `EXTEVNT10` reader - External Event 10"]
pub use EXTEVNT1_R as EXTEVNT10_R;
#[doc = "Field `EXTEVNT2` writer - External Event 2"]
pub use EXTEVNT1_W as EXTEVNT2_W;
#[doc = "Field `EXTEVNT3` writer - External Event 3"]
pub use EXTEVNT1_W as EXTEVNT3_W;
#[doc = "Field `EXTEVNT4` writer - External Event 4"]
pub use EXTEVNT1_W as EXTEVNT4_W;
#[doc = "Field `EXTEVNT5` writer - External Event 5"]
pub use EXTEVNT1_W as EXTEVNT5_W;
#[doc = "Field `EXTEVNT6` writer - External Event 6"]
pub use EXTEVNT1_W as EXTEVNT6_W;
#[doc = "Field `EXTEVNT7` writer - External Event 7"]
pub use EXTEVNT1_W as EXTEVNT7_W;
#[doc = "Field `EXTEVNT8` writer - External Event 8"]
pub use EXTEVNT1_W as EXTEVNT8_W;
#[doc = "Field `EXTEVNT9` writer - External Event 9"]
pub use EXTEVNT1_W as EXTEVNT9_W;
#[doc = "Field `EXTEVNT10` writer - External Event 10"]
pub use EXTEVNT1_W as EXTEVNT10_W;
#[doc = "Field `TIMACMP1` reader - Timer A Compare 1"]
pub type TIMACMP1_R = crate::BitReader<TIMACMP1_A>;
#[doc = "Timer A Compare 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMACMP1_A {
    #[doc = "0: Timer Y compare Z event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X counter is reset upon timer Y compare Z event"]
    ResetCounter = 1,
}
impl From<TIMACMP1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMACMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMACMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMACMP1_A {
        match self.bits {
            false => TIMACMP1_A::NoEffect,
            true => TIMACMP1_A::ResetCounter,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIMACMP1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `ResetCounter`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == TIMACMP1_A::ResetCounter
    }
}
#[doc = "Field `TIMACMP1` writer - Timer A Compare 1"]
pub type TIMACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTBR_SPEC, TIMACMP1_A, O>;
impl<'a, const O: u8> TIMACMP1_W<'a, O> {
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMACMP1_A::NoEffect)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMACMP1_A::ResetCounter)
    }
}
#[doc = "Field `TIMACMP2` reader - Timer A Compare 2"]
pub use TIMACMP1_R as TIMACMP2_R;
#[doc = "Field `TIMACMP4` reader - Timer A Compare 4"]
pub use TIMACMP1_R as TIMACMP4_R;
#[doc = "Field `TIMCCMP1` reader - Timer C Compare 1"]
pub use TIMACMP1_R as TIMCCMP1_R;
#[doc = "Field `TIMCCMP2` reader - Timer C Compare 2"]
pub use TIMACMP1_R as TIMCCMP2_R;
#[doc = "Field `TIMCCMP4` reader - Timer C Compare 4"]
pub use TIMACMP1_R as TIMCCMP4_R;
#[doc = "Field `TIMDCMP1` reader - Timer D Compare 1"]
pub use TIMACMP1_R as TIMDCMP1_R;
#[doc = "Field `TIMDCMP2` reader - Timer D Compare 2"]
pub use TIMACMP1_R as TIMDCMP2_R;
#[doc = "Field `TIMDCMP4` reader - Timer D Compare 4"]
pub use TIMACMP1_R as TIMDCMP4_R;
#[doc = "Field `TIMECMP1` reader - Timer E Compare 1"]
pub use TIMACMP1_R as TIMECMP1_R;
#[doc = "Field `TIMECMP2` reader - Timer E Compare 2"]
pub use TIMACMP1_R as TIMECMP2_R;
#[doc = "Field `TIMECMP4` reader - Timer E Compare 4"]
pub use TIMACMP1_R as TIMECMP4_R;
#[doc = "Field `TIMACMP2` writer - Timer A Compare 2"]
pub use TIMACMP1_W as TIMACMP2_W;
#[doc = "Field `TIMACMP4` writer - Timer A Compare 4"]
pub use TIMACMP1_W as TIMACMP4_W;
#[doc = "Field `TIMCCMP1` writer - Timer C Compare 1"]
pub use TIMACMP1_W as TIMCCMP1_W;
#[doc = "Field `TIMCCMP2` writer - Timer C Compare 2"]
pub use TIMACMP1_W as TIMCCMP2_W;
#[doc = "Field `TIMCCMP4` writer - Timer C Compare 4"]
pub use TIMACMP1_W as TIMCCMP4_W;
#[doc = "Field `TIMDCMP1` writer - Timer D Compare 1"]
pub use TIMACMP1_W as TIMDCMP1_W;
#[doc = "Field `TIMDCMP2` writer - Timer D Compare 2"]
pub use TIMACMP1_W as TIMDCMP2_W;
#[doc = "Field `TIMDCMP4` writer - Timer D Compare 4"]
pub use TIMACMP1_W as TIMDCMP4_W;
#[doc = "Field `TIMECMP1` writer - Timer E Compare 1"]
pub use TIMACMP1_W as TIMECMP1_W;
#[doc = "Field `TIMECMP2` writer - Timer E Compare 2"]
pub use TIMACMP1_W as TIMECMP2_W;
#[doc = "Field `TIMECMP4` writer - Timer E Compare 4"]
pub use TIMACMP1_W as TIMECMP4_W;
impl R {
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    pub fn updt(&self) -> UPDT_R {
        UPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer A Compare 1"]
    #[inline(always)]
    pub fn timacmp1(&self) -> TIMACMP1_R {
        TIMACMP1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer A Compare 2"]
    #[inline(always)]
    pub fn timacmp2(&self) -> TIMACMP2_R {
        TIMACMP2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer A Compare 4"]
    #[inline(always)]
    pub fn timacmp4(&self) -> TIMACMP4_R {
        TIMACMP4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn timccmp1(&self) -> TIMCCMP1_R {
        TIMCCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn timccmp2(&self) -> TIMCCMP2_R {
        TIMCCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timer C Compare 4"]
    #[inline(always)]
    pub fn timccmp4(&self) -> TIMCCMP4_R {
        TIMCCMP4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    pub fn timdcmp1(&self) -> TIMDCMP1_R {
        TIMDCMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    pub fn timdcmp2(&self) -> TIMDCMP2_R {
        TIMDCMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    pub fn timdcmp4(&self) -> TIMDCMP4_R {
        TIMDCMP4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    pub fn timecmp1(&self) -> TIMECMP1_R {
        TIMECMP1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    pub fn timecmp2(&self) -> TIMECMP2_R {
        TIMECMP2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    pub fn timecmp4(&self) -> TIMECMP4_R {
        TIMECMP4_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    #[must_use]
    pub fn updt(&mut self) -> UPDT_W<1> {
        UPDT_W::new(self)
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<2> {
        CMP2_W::new(self)
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4(&mut self) -> CMP4_W<3> {
        CMP4_W::new(self)
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    #[must_use]
    pub fn mstper(&mut self) -> MSTPER_W<4> {
        MSTPER_W::new(self)
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<5> {
        MSTCMP1_W::new(self)
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<6> {
        MSTCMP2_W::new(self)
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<7> {
        MSTCMP3_W::new(self)
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<8> {
        MSTCMP4_W::new(self)
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<9> {
        EXTEVNT1_W::new(self)
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<10> {
        EXTEVNT2_W::new(self)
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<11> {
        EXTEVNT3_W::new(self)
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<12> {
        EXTEVNT4_W::new(self)
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<13> {
        EXTEVNT5_W::new(self)
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<14> {
        EXTEVNT6_W::new(self)
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<15> {
        EXTEVNT7_W::new(self)
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<16> {
        EXTEVNT8_W::new(self)
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<17> {
        EXTEVNT9_W::new(self)
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    #[must_use]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<18> {
        EXTEVNT10_W::new(self)
    }
    #[doc = "Bit 19 - Timer A Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn timacmp1(&mut self) -> TIMACMP1_W<19> {
        TIMACMP1_W::new(self)
    }
    #[doc = "Bit 20 - Timer A Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timacmp2(&mut self) -> TIMACMP2_W<20> {
        TIMACMP2_W::new(self)
    }
    #[doc = "Bit 21 - Timer A Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn timacmp4(&mut self) -> TIMACMP4_W<21> {
        TIMACMP4_W::new(self)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn timccmp1(&mut self) -> TIMCCMP1_W<22> {
        TIMCCMP1_W::new(self)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timccmp2(&mut self) -> TIMCCMP2_W<23> {
        TIMCCMP2_W::new(self)
    }
    #[doc = "Bit 24 - Timer C Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn timccmp4(&mut self) -> TIMCCMP4_W<24> {
        TIMCCMP4_W::new(self)
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn timdcmp1(&mut self) -> TIMDCMP1_W<25> {
        TIMDCMP1_W::new(self)
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timdcmp2(&mut self) -> TIMDCMP2_W<26> {
        TIMDCMP2_W::new(self)
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn timdcmp4(&mut self) -> TIMDCMP4_W<27> {
        TIMDCMP4_W::new(self)
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn timecmp1(&mut self) -> TIMECMP1_W<28> {
        TIMECMP1_W::new(self)
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn timecmp2(&mut self) -> TIMECMP2_W<29> {
        TIMECMP2_W::new(self)
    }
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    #[must_use]
    pub fn timecmp4(&mut self) -> TIMECMP4_W<30> {
        TIMECMP4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstbr](index.html) module"]
pub struct RSTBR_SPEC;
impl crate::RegisterSpec for RSTBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstbr::R](R) reader structure"]
impl crate::Readable for RSTBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstbr::W](W) writer structure"]
impl crate::Writable for RSTBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTBR to value 0"]
impl crate::Resettable for RSTBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
