#[doc = "Register `CPT2DCR` reader"]
pub struct R(crate::R<CPT2DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT2DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT2DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT2DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPT2DCR` writer"]
pub struct W(crate::W<CPT2DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPT2DCR_SPEC>;
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
impl From<crate::W<CPT2DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPT2DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWCPT` reader - Software Capture"]
pub type SWCPT_R = crate::BitReader<SWCPT_A>;
#[doc = "Software Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWCPT_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Force capture Z"]
    TriggerCapture = 1,
}
impl From<SWCPT_A> for bool {
    #[inline(always)]
    fn from(variant: SWCPT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWCPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWCPT_A {
        match self.bits {
            false => SWCPT_A::NoEffect,
            true => SWCPT_A::TriggerCapture,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SWCPT_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `TriggerCapture`"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == SWCPT_A::TriggerCapture
    }
}
#[doc = "Field `SWCPT` writer - Software Capture"]
pub type SWCPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT2DCR_SPEC, SWCPT_A, O>;
impl<'a, const O: u8> SWCPT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SWCPT_A::NoEffect)
    }
    #[doc = "Force capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut W {
        self.variant(SWCPT_A::TriggerCapture)
    }
}
#[doc = "Field `UPDCPT` reader - Update Capture"]
pub type UPDCPT_R = crate::BitReader<UPDCPT_A>;
#[doc = "Update Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDCPT_A {
    #[doc = "0: Update event has no effect"]
    NoEffect = 0,
    #[doc = "1: Update event triggers capture Z"]
    TriggerCapture = 1,
}
impl From<UPDCPT_A> for bool {
    #[inline(always)]
    fn from(variant: UPDCPT_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDCPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDCPT_A {
        match self.bits {
            false => UPDCPT_A::NoEffect,
            true => UPDCPT_A::TriggerCapture,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDCPT_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `TriggerCapture`"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == UPDCPT_A::TriggerCapture
    }
}
#[doc = "Field `UPDCPT` writer - Update Capture"]
pub type UPDCPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT2DCR_SPEC, UPDCPT_A, O>;
impl<'a, const O: u8> UPDCPT_W<'a, O> {
    #[doc = "Update event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UPDCPT_A::NoEffect)
    }
    #[doc = "Update event triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut W {
        self.variant(UPDCPT_A::TriggerCapture)
    }
}
#[doc = "Field `EXEV1CPT` reader - External Event 1 Capture"]
pub type EXEV1CPT_R = crate::BitReader<EXEV1CPT_A>;
#[doc = "External Event 1 Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXEV1CPT_A {
    #[doc = "0: External event Y has no effect"]
    NoEffect = 0,
    #[doc = "1: External event Y triggers capture Z"]
    TriggerCapture = 1,
}
impl From<EXEV1CPT_A> for bool {
    #[inline(always)]
    fn from(variant: EXEV1CPT_A) -> Self {
        variant as u8 != 0
    }
}
impl EXEV1CPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXEV1CPT_A {
        match self.bits {
            false => EXEV1CPT_A::NoEffect,
            true => EXEV1CPT_A::TriggerCapture,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXEV1CPT_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `TriggerCapture`"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == EXEV1CPT_A::TriggerCapture
    }
}
#[doc = "Field `EXEV1CPT` writer - External Event 1 Capture"]
pub type EXEV1CPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT2DCR_SPEC, EXEV1CPT_A, O>;
impl<'a, const O: u8> EXEV1CPT_W<'a, O> {
    #[doc = "External event Y has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXEV1CPT_A::NoEffect)
    }
    #[doc = "External event Y triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut W {
        self.variant(EXEV1CPT_A::TriggerCapture)
    }
}
#[doc = "Field `EXEV2CPT` reader - External Event 2 Capture"]
pub use EXEV1CPT_R as EXEV2CPT_R;
#[doc = "Field `EXEV3CPT` reader - External Event 3 Capture"]
pub use EXEV1CPT_R as EXEV3CPT_R;
#[doc = "Field `EXEV4CPT` reader - External Event 4 Capture"]
pub use EXEV1CPT_R as EXEV4CPT_R;
#[doc = "Field `EXEV5CPT` reader - External Event 5 Capture"]
pub use EXEV1CPT_R as EXEV5CPT_R;
#[doc = "Field `EXEV6CPT` reader - External Event 6 Capture"]
pub use EXEV1CPT_R as EXEV6CPT_R;
#[doc = "Field `EXEV7CPT` reader - External Event 7 Capture"]
pub use EXEV1CPT_R as EXEV7CPT_R;
#[doc = "Field `EXEV8CPT` reader - External Event 8 Capture"]
pub use EXEV1CPT_R as EXEV8CPT_R;
#[doc = "Field `EXEV9CPT` reader - External Event 9 Capture"]
pub use EXEV1CPT_R as EXEV9CPT_R;
#[doc = "Field `EXEV10CPT` reader - External Event 10 Capture"]
pub use EXEV1CPT_R as EXEV10CPT_R;
#[doc = "Field `EXEV2CPT` writer - External Event 2 Capture"]
pub use EXEV1CPT_W as EXEV2CPT_W;
#[doc = "Field `EXEV3CPT` writer - External Event 3 Capture"]
pub use EXEV1CPT_W as EXEV3CPT_W;
#[doc = "Field `EXEV4CPT` writer - External Event 4 Capture"]
pub use EXEV1CPT_W as EXEV4CPT_W;
#[doc = "Field `EXEV5CPT` writer - External Event 5 Capture"]
pub use EXEV1CPT_W as EXEV5CPT_W;
#[doc = "Field `EXEV6CPT` writer - External Event 6 Capture"]
pub use EXEV1CPT_W as EXEV6CPT_W;
#[doc = "Field `EXEV7CPT` writer - External Event 7 Capture"]
pub use EXEV1CPT_W as EXEV7CPT_W;
#[doc = "Field `EXEV8CPT` writer - External Event 8 Capture"]
pub use EXEV1CPT_W as EXEV8CPT_W;
#[doc = "Field `EXEV9CPT` writer - External Event 9 Capture"]
pub use EXEV1CPT_W as EXEV9CPT_W;
#[doc = "Field `EXEV10CPT` writer - External Event 10 Capture"]
pub use EXEV1CPT_W as EXEV10CPT_W;
#[doc = "Field `TA1SET` reader - Timer A output 1 Set"]
pub type TA1SET_R = crate::BitReader<TA1SET_A>;
#[doc = "Timer A output 1 Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1SET_A {
    #[doc = "0: Timer X output Y inactive to active transition has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X output Y inactive to active transition triggers capture Z"]
    TriggerCapture = 1,
}
impl From<TA1SET_A> for bool {
    #[inline(always)]
    fn from(variant: TA1SET_A) -> Self {
        variant as u8 != 0
    }
}
impl TA1SET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TA1SET_A {
        match self.bits {
            false => TA1SET_A::NoEffect,
            true => TA1SET_A::TriggerCapture,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TA1SET_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `TriggerCapture`"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TA1SET_A::TriggerCapture
    }
}
#[doc = "Field `TA1SET` writer - Timer A output 1 Set"]
pub type TA1SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT2DCR_SPEC, TA1SET_A, O>;
impl<'a, const O: u8> TA1SET_W<'a, O> {
    #[doc = "Timer X output Y inactive to active transition has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TA1SET_A::NoEffect)
    }
    #[doc = "Timer X output Y inactive to active transition triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut W {
        self.variant(TA1SET_A::TriggerCapture)
    }
}
#[doc = "Field `TA1RST` reader - Timer A output 1 Reset"]
pub type TA1RST_R = crate::BitReader<TA1RST_A>;
#[doc = "Timer A output 1 Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1RST_A {
    #[doc = "0: Timer X output Y active to inactive transition has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X output Y active to inactive transition triggers capture Z"]
    TriggerCapture = 1,
}
impl From<TA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TA1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TA1RST_A {
        match self.bits {
            false => TA1RST_A::NoEffect,
            true => TA1RST_A::TriggerCapture,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TA1RST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `TriggerCapture`"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TA1RST_A::TriggerCapture
    }
}
#[doc = "Field `TA1RST` writer - Timer A output 1 Reset"]
pub type TA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT2DCR_SPEC, TA1RST_A, O>;
impl<'a, const O: u8> TA1RST_W<'a, O> {
    #[doc = "Timer X output Y active to inactive transition has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TA1RST_A::NoEffect)
    }
    #[doc = "Timer X output Y active to inactive transition triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut W {
        self.variant(TA1RST_A::TriggerCapture)
    }
}
#[doc = "Field `TACMP1` reader - Timer A Compare 1"]
pub type TACMP1_R = crate::BitReader<TACMP1_A>;
#[doc = "Timer A Compare 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TACMP1_A {
    #[doc = "0: Timer X compare Y has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X compare Y triggers capture Z"]
    TriggerCapture = 1,
}
impl From<TACMP1_A> for bool {
    #[inline(always)]
    fn from(variant: TACMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl TACMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TACMP1_A {
        match self.bits {
            false => TACMP1_A::NoEffect,
            true => TACMP1_A::TriggerCapture,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TACMP1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `TriggerCapture`"]
    #[inline(always)]
    pub fn is_trigger_capture(&self) -> bool {
        *self == TACMP1_A::TriggerCapture
    }
}
#[doc = "Field `TACMP1` writer - Timer A Compare 1"]
pub type TACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPT2DCR_SPEC, TACMP1_A, O>;
impl<'a, const O: u8> TACMP1_W<'a, O> {
    #[doc = "Timer X compare Y has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TACMP1_A::NoEffect)
    }
    #[doc = "Timer X compare Y triggers capture Z"]
    #[inline(always)]
    pub fn trigger_capture(self) -> &'a mut W {
        self.variant(TACMP1_A::TriggerCapture)
    }
}
#[doc = "Field `TB1RST` reader - Timer B output 1 Reset"]
pub use TA1RST_R as TB1RST_R;
#[doc = "Field `TC1RST` reader - Timer C output 1 Reset"]
pub use TA1RST_R as TC1RST_R;
#[doc = "Field `TE1RST` reader - Timer E output 1 Reset"]
pub use TA1RST_R as TE1RST_R;
#[doc = "Field `TB1RST` writer - Timer B output 1 Reset"]
pub use TA1RST_W as TB1RST_W;
#[doc = "Field `TC1RST` writer - Timer C output 1 Reset"]
pub use TA1RST_W as TC1RST_W;
#[doc = "Field `TE1RST` writer - Timer E output 1 Reset"]
pub use TA1RST_W as TE1RST_W;
#[doc = "Field `TB1SET` reader - Timer B output 1 Set"]
pub use TA1SET_R as TB1SET_R;
#[doc = "Field `TC1SET` reader - Timer C output 1 Set"]
pub use TA1SET_R as TC1SET_R;
#[doc = "Field `TE1SET` reader - Timer E output 1 Set"]
pub use TA1SET_R as TE1SET_R;
#[doc = "Field `TB1SET` writer - Timer B output 1 Set"]
pub use TA1SET_W as TB1SET_W;
#[doc = "Field `TC1SET` writer - Timer C output 1 Set"]
pub use TA1SET_W as TC1SET_W;
#[doc = "Field `TE1SET` writer - Timer E output 1 Set"]
pub use TA1SET_W as TE1SET_W;
#[doc = "Field `TACMP2` reader - Timer A Compare 2"]
pub use TACMP1_R as TACMP2_R;
#[doc = "Field `TBCMP1` reader - Timer B Compare 1"]
pub use TACMP1_R as TBCMP1_R;
#[doc = "Field `TBCMP2` reader - Timer B Compare 2"]
pub use TACMP1_R as TBCMP2_R;
#[doc = "Field `TCCMP1` reader - Timer C Compare 1"]
pub use TACMP1_R as TCCMP1_R;
#[doc = "Field `TCCMP2` reader - Timer C Compare 2"]
pub use TACMP1_R as TCCMP2_R;
#[doc = "Field `TECMP1` reader - Timer E Compare 1"]
pub use TACMP1_R as TECMP1_R;
#[doc = "Field `TECMP2` reader - Timer E Compare 2"]
pub use TACMP1_R as TECMP2_R;
#[doc = "Field `TACMP2` writer - Timer A Compare 2"]
pub use TACMP1_W as TACMP2_W;
#[doc = "Field `TBCMP1` writer - Timer B Compare 1"]
pub use TACMP1_W as TBCMP1_W;
#[doc = "Field `TBCMP2` writer - Timer B Compare 2"]
pub use TACMP1_W as TBCMP2_W;
#[doc = "Field `TCCMP1` writer - Timer C Compare 1"]
pub use TACMP1_W as TCCMP1_W;
#[doc = "Field `TCCMP2` writer - Timer C Compare 2"]
pub use TACMP1_W as TCCMP2_W;
#[doc = "Field `TECMP1` writer - Timer E Compare 1"]
pub use TACMP1_W as TECMP1_W;
#[doc = "Field `TECMP2` writer - Timer E Compare 2"]
pub use TACMP1_W as TECMP2_W;
impl R {
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    pub fn swcpt(&self) -> SWCPT_R {
        SWCPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    pub fn updcpt(&self) -> UPDCPT_R {
        UPDCPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    pub fn exev1cpt(&self) -> EXEV1CPT_R {
        EXEV1CPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    pub fn exev2cpt(&self) -> EXEV2CPT_R {
        EXEV2CPT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    pub fn exev3cpt(&self) -> EXEV3CPT_R {
        EXEV3CPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    pub fn exev4cpt(&self) -> EXEV4CPT_R {
        EXEV4CPT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    pub fn exev5cpt(&self) -> EXEV5CPT_R {
        EXEV5CPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    pub fn exev6cpt(&self) -> EXEV6CPT_R {
        EXEV6CPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    pub fn exev7cpt(&self) -> EXEV7CPT_R {
        EXEV7CPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    pub fn exev8cpt(&self) -> EXEV8CPT_R {
        EXEV8CPT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    pub fn exev9cpt(&self) -> EXEV9CPT_R {
        EXEV9CPT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    pub fn exev10cpt(&self) -> EXEV10CPT_R {
        EXEV10CPT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer A output 1 Set"]
    #[inline(always)]
    pub fn ta1set(&self) -> TA1SET_R {
        TA1SET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer A output 1 Reset"]
    #[inline(always)]
    pub fn ta1rst(&self) -> TA1RST_R {
        TA1RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer A Compare 1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer A Compare 2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    pub fn tb1set(&self) -> TB1SET_R {
        TB1SET_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    pub fn tb1rst(&self) -> TB1RST_R {
        TB1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer C output 1 Set"]
    #[inline(always)]
    pub fn tc1set(&self) -> TC1SET_R {
        TC1SET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer C output 1 Reset"]
    #[inline(always)]
    pub fn tc1rst(&self) -> TC1RST_R {
        TC1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer E output 1 Set"]
    #[inline(always)]
    pub fn te1set(&self) -> TE1SET_R {
        TE1SET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timer E output 1 Reset"]
    #[inline(always)]
    pub fn te1rst(&self) -> TE1RST_R {
        TE1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Timer E Compare 1"]
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Timer E Compare 2"]
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    #[must_use]
    pub fn swcpt(&mut self) -> SWCPT_W<0> {
        SWCPT_W::new(self)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    #[must_use]
    pub fn updcpt(&mut self) -> UPDCPT_W<1> {
        UPDCPT_W::new(self)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev1cpt(&mut self) -> EXEV1CPT_W<2> {
        EXEV1CPT_W::new(self)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev2cpt(&mut self) -> EXEV2CPT_W<3> {
        EXEV2CPT_W::new(self)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev3cpt(&mut self) -> EXEV3CPT_W<4> {
        EXEV3CPT_W::new(self)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev4cpt(&mut self) -> EXEV4CPT_W<5> {
        EXEV4CPT_W::new(self)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev5cpt(&mut self) -> EXEV5CPT_W<6> {
        EXEV5CPT_W::new(self)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev6cpt(&mut self) -> EXEV6CPT_W<7> {
        EXEV6CPT_W::new(self)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev7cpt(&mut self) -> EXEV7CPT_W<8> {
        EXEV7CPT_W::new(self)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev8cpt(&mut self) -> EXEV8CPT_W<9> {
        EXEV8CPT_W::new(self)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev9cpt(&mut self) -> EXEV9CPT_W<10> {
        EXEV9CPT_W::new(self)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    #[must_use]
    pub fn exev10cpt(&mut self) -> EXEV10CPT_W<11> {
        EXEV10CPT_W::new(self)
    }
    #[doc = "Bit 12 - Timer A output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn ta1set(&mut self) -> TA1SET_W<12> {
        TA1SET_W::new(self)
    }
    #[doc = "Bit 13 - Timer A output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ta1rst(&mut self) -> TA1RST_W<13> {
        TA1RST_W::new(self)
    }
    #[doc = "Bit 14 - Timer A Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp1(&mut self) -> TACMP1_W<14> {
        TACMP1_W::new(self)
    }
    #[doc = "Bit 15 - Timer A Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp2(&mut self) -> TACMP2_W<15> {
        TACMP2_W::new(self)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn tb1set(&mut self) -> TB1SET_W<16> {
        TB1SET_W::new(self)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tb1rst(&mut self) -> TB1RST_W<17> {
        TB1RST_W::new(self)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<18> {
        TBCMP1_W::new(self)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<19> {
        TBCMP2_W::new(self)
    }
    #[doc = "Bit 20 - Timer C output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn tc1set(&mut self) -> TC1SET_W<20> {
        TC1SET_W::new(self)
    }
    #[doc = "Bit 21 - Timer C output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tc1rst(&mut self) -> TC1RST_W<21> {
        TC1RST_W::new(self)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp1(&mut self) -> TCCMP1_W<22> {
        TCCMP1_W::new(self)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp2(&mut self) -> TCCMP2_W<23> {
        TCCMP2_W::new(self)
    }
    #[doc = "Bit 28 - Timer E output 1 Set"]
    #[inline(always)]
    #[must_use]
    pub fn te1set(&mut self) -> TE1SET_W<28> {
        TE1SET_W::new(self)
    }
    #[doc = "Bit 29 - Timer E output 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn te1rst(&mut self) -> TE1RST_W<29> {
        TE1RST_W::new(self)
    }
    #[doc = "Bit 30 - Timer E Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp1(&mut self) -> TECMP1_W<30> {
        TECMP1_W::new(self)
    }
    #[doc = "Bit 31 - Timer E Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp2(&mut self) -> TECMP2_W<31> {
        TECMP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPT2xCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2dcr](index.html) module"]
pub struct CPT2DCR_SPEC;
impl crate::RegisterSpec for CPT2DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpt2dcr::R](R) reader structure"]
impl crate::Readable for CPT2DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpt2dcr::W](W) writer structure"]
impl crate::Writable for CPT2DCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPT2DCR to value 0"]
impl crate::Resettable for CPT2DCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
