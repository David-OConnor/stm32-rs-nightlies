#[doc = "Register `BMTRGR` reader"]
pub struct R(crate::R<BMTRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMTRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMTRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMTRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMTRGR` writer"]
pub struct W(crate::W<BMTRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMTRGR_SPEC>;
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
impl From<crate::W<BMTRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMTRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - SW"]
pub type SW_R = crate::BitReader<SW_A>;
#[doc = "SW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Trigger immediate burst mode operation"]
    Trigger = 1,
}
impl From<SW_A> for bool {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as u8 != 0
    }
}
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            false => SW_A::NoEffect,
            true => SW_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SW_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SW_A::Trigger
    }
}
#[doc = "Field `SW` writer - SW"]
pub type SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, SW_A, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SW_A::NoEffect)
    }
    #[doc = "Trigger immediate burst mode operation"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(SW_A::Trigger)
    }
}
#[doc = "Field `MSTRST` reader - MSTRST"]
pub type MSTRST_R = crate::BitReader<MSTRST_A>;
#[doc = "MSTRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTRST_A {
    #[doc = "0: Master timer reset/roll-over event has no effect"]
    NoEffect = 0,
    #[doc = "1: Master timer reset/roll-over event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<MSTRST_A> for bool {
    #[inline(always)]
    fn from(variant: MSTRST_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTRST_A {
        match self.bits {
            false => MSTRST_A::NoEffect,
            true => MSTRST_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTRST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTRST_A::Trigger
    }
}
#[doc = "Field `MSTRST` writer - MSTRST"]
pub type MSTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, MSTRST_A, O>;
impl<'a, const O: u8> MSTRST_W<'a, O> {
    #[doc = "Master timer reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTRST_A::NoEffect)
    }
    #[doc = "Master timer reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTRST_A::Trigger)
    }
}
#[doc = "Field `MSTREP` reader - MSTREP"]
pub type MSTREP_R = crate::BitReader<MSTREP_A>;
#[doc = "MSTREP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTREP_A {
    #[doc = "0: Master timer repetition event has no effect"]
    NoEffect = 0,
    #[doc = "1: Master timer repetition event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<MSTREP_A> for bool {
    #[inline(always)]
    fn from(variant: MSTREP_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTREP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTREP_A {
        match self.bits {
            false => MSTREP_A::NoEffect,
            true => MSTREP_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTREP_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTREP_A::Trigger
    }
}
#[doc = "Field `MSTREP` writer - MSTREP"]
pub type MSTREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, MSTREP_A, O>;
impl<'a, const O: u8> MSTREP_W<'a, O> {
    #[doc = "Master timer repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTREP_A::NoEffect)
    }
    #[doc = "Master timer repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTREP_A::Trigger)
    }
}
#[doc = "Field `MSTCMP1` reader - MSTCMP1"]
pub type MSTCMP1_R = crate::BitReader<MSTCMP1_A>;
#[doc = "MSTCMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCMP1_A {
    #[doc = "0: Master timer compare X event has no effect"]
    NoEffect = 0,
    #[doc = "1: Master timer compare X event triggers a burst mode entry"]
    Trigger = 1,
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
            true => MSTCMP1_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTCMP1_A::Trigger
    }
}
#[doc = "Field `MSTCMP1` writer - MSTCMP1"]
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, MSTCMP1_A, O>;
impl<'a, const O: u8> MSTCMP1_W<'a, O> {
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP1_A::NoEffect)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTCMP1_A::Trigger)
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
#[doc = "Field `TARST` reader - TARST"]
pub type TARST_R = crate::BitReader<TARST_A>;
#[doc = "TARST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TARST_A {
    #[doc = "0: Timer X reset/roll-over event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X reset/roll-over event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<TARST_A> for bool {
    #[inline(always)]
    fn from(variant: TARST_A) -> Self {
        variant as u8 != 0
    }
}
impl TARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARST_A {
        match self.bits {
            false => TARST_A::NoEffect,
            true => TARST_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TARST_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TARST_A::Trigger
    }
}
#[doc = "Field `TARST` writer - TARST"]
pub type TARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, TARST_A, O>;
impl<'a, const O: u8> TARST_W<'a, O> {
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TARST_A::NoEffect)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TARST_A::Trigger)
    }
}
#[doc = "Field `TAREP` reader - TAREP"]
pub type TAREP_R = crate::BitReader<TAREP_A>;
#[doc = "TAREP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAREP_A {
    #[doc = "0: Timer X repetition event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X repetition event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<TAREP_A> for bool {
    #[inline(always)]
    fn from(variant: TAREP_A) -> Self {
        variant as u8 != 0
    }
}
impl TAREP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAREP_A {
        match self.bits {
            false => TAREP_A::NoEffect,
            true => TAREP_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TAREP_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TAREP_A::Trigger
    }
}
#[doc = "Field `TAREP` writer - TAREP"]
pub type TAREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, TAREP_A, O>;
impl<'a, const O: u8> TAREP_W<'a, O> {
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TAREP_A::NoEffect)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TAREP_A::Trigger)
    }
}
#[doc = "Field `TACMP1` reader - TACMP1"]
pub type TACMP1_R = crate::BitReader<TACMP1_A>;
#[doc = "TACMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TACMP1_A {
    #[doc = "0: Timer X compare Y event has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X compare Y event triggers a burst mode entry"]
    Trigger = 1,
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
            true => TACMP1_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TACMP1_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TACMP1_A::Trigger
    }
}
#[doc = "Field `TACMP1` writer - TACMP1"]
pub type TACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, TACMP1_A, O>;
impl<'a, const O: u8> TACMP1_W<'a, O> {
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TACMP1_A::NoEffect)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TACMP1_A::Trigger)
    }
}
#[doc = "Field `TACMP2` reader - TACMP2"]
pub use TACMP1_R as TACMP2_R;
#[doc = "Field `TBCMP1` reader - TBCMP1"]
pub use TACMP1_R as TBCMP1_R;
#[doc = "Field `TBCMP2` reader - TBCMP2"]
pub use TACMP1_R as TBCMP2_R;
#[doc = "Field `TCCMP1` reader - TCCMP1"]
pub use TACMP1_R as TCCMP1_R;
#[doc = "Field `TCCMP2` reader - TCCMP2"]
pub use TACMP1_R as TCCMP2_R;
#[doc = "Field `TDCMP1` reader - TDCMP1"]
pub use TACMP1_R as TDCMP1_R;
#[doc = "Field `TDCMP2` reader - TDCMP2"]
pub use TACMP1_R as TDCMP2_R;
#[doc = "Field `TECMP1` reader - TECMP1"]
pub use TACMP1_R as TECMP1_R;
#[doc = "Field `TECMP2` reader - TECMP2"]
pub use TACMP1_R as TECMP2_R;
#[doc = "Field `TACMP2` writer - TACMP2"]
pub use TACMP1_W as TACMP2_W;
#[doc = "Field `TBCMP1` writer - TBCMP1"]
pub use TACMP1_W as TBCMP1_W;
#[doc = "Field `TBCMP2` writer - TBCMP2"]
pub use TACMP1_W as TBCMP2_W;
#[doc = "Field `TCCMP1` writer - TCCMP1"]
pub use TACMP1_W as TCCMP1_W;
#[doc = "Field `TCCMP2` writer - TCCMP2"]
pub use TACMP1_W as TCCMP2_W;
#[doc = "Field `TDCMP1` writer - TDCMP1"]
pub use TACMP1_W as TDCMP1_W;
#[doc = "Field `TDCMP2` writer - TDCMP2"]
pub use TACMP1_W as TDCMP2_W;
#[doc = "Field `TECMP1` writer - TECMP1"]
pub use TACMP1_W as TECMP1_W;
#[doc = "Field `TECMP2` writer - TECMP2"]
pub use TACMP1_W as TECMP2_W;
#[doc = "Field `TBREP` reader - TBREP"]
pub use TAREP_R as TBREP_R;
#[doc = "Field `TCREP` reader - TCREP"]
pub use TAREP_R as TCREP_R;
#[doc = "Field `TDREP` reader - TDREP"]
pub use TAREP_R as TDREP_R;
#[doc = "Field `TEREP` reader - TEREP"]
pub use TAREP_R as TEREP_R;
#[doc = "Field `TBREP` writer - TBREP"]
pub use TAREP_W as TBREP_W;
#[doc = "Field `TCREP` writer - TCREP"]
pub use TAREP_W as TCREP_W;
#[doc = "Field `TDREP` writer - TDREP"]
pub use TAREP_W as TDREP_W;
#[doc = "Field `TEREP` writer - TEREP"]
pub use TAREP_W as TEREP_W;
#[doc = "Field `TBRST` reader - TBRST"]
pub use TARST_R as TBRST_R;
#[doc = "Field `TCRST` reader - TCRST"]
pub use TARST_R as TCRST_R;
#[doc = "Field `TDRST` reader - TDRST"]
pub use TARST_R as TDRST_R;
#[doc = "Field `TERST` reader - TERST"]
pub use TARST_R as TERST_R;
#[doc = "Field `TBRST` writer - TBRST"]
pub use TARST_W as TBRST_W;
#[doc = "Field `TCRST` writer - TCRST"]
pub use TARST_W as TCRST_W;
#[doc = "Field `TDRST` writer - TDRST"]
pub use TARST_W as TDRST_W;
#[doc = "Field `TERST` writer - TERST"]
pub use TARST_W as TERST_W;
#[doc = "Field `TAEEV7` reader - TAEEV7"]
pub type TAEEV7_R = crate::BitReader<TAEEV7_A>;
#[doc = "TAEEV7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAEEV7_A {
    #[doc = "0: Timer X period following external event Y has no effect"]
    NoEffect = 0,
    #[doc = "1: Timer X period following external event Y triggers a burst mode entry"]
    Trigger = 1,
}
impl From<TAEEV7_A> for bool {
    #[inline(always)]
    fn from(variant: TAEEV7_A) -> Self {
        variant as u8 != 0
    }
}
impl TAEEV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAEEV7_A {
        match self.bits {
            false => TAEEV7_A::NoEffect,
            true => TAEEV7_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TAEEV7_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TAEEV7_A::Trigger
    }
}
#[doc = "Field `TAEEV7` writer - TAEEV7"]
pub type TAEEV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, TAEEV7_A, O>;
impl<'a, const O: u8> TAEEV7_W<'a, O> {
    #[doc = "Timer X period following external event Y has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TAEEV7_A::NoEffect)
    }
    #[doc = "Timer X period following external event Y triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TAEEV7_A::Trigger)
    }
}
#[doc = "Field `TDEEV8` reader - TDEEV8"]
pub use TAEEV7_R as TDEEV8_R;
#[doc = "Field `TDEEV8` writer - TDEEV8"]
pub use TAEEV7_W as TDEEV8_W;
#[doc = "Field `EEV7` reader - EEV7"]
pub type EEV7_R = crate::BitReader<EEV7_A>;
#[doc = "EEV7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEV7_A {
    #[doc = "0: External event X has no effect"]
    NoEffect = 0,
    #[doc = "1: External event X triggers a burst mode entry"]
    Trigger = 1,
}
impl From<EEV7_A> for bool {
    #[inline(always)]
    fn from(variant: EEV7_A) -> Self {
        variant as u8 != 0
    }
}
impl EEV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEV7_A {
        match self.bits {
            false => EEV7_A::NoEffect,
            true => EEV7_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EEV7_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == EEV7_A::Trigger
    }
}
#[doc = "Field `EEV7` writer - EEV7"]
pub type EEV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, EEV7_A, O>;
impl<'a, const O: u8> EEV7_W<'a, O> {
    #[doc = "External event X has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EEV7_A::NoEffect)
    }
    #[doc = "External event X triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(EEV7_A::Trigger)
    }
}
#[doc = "Field `EEV8` reader - EEV8"]
pub use EEV7_R as EEV8_R;
#[doc = "Field `EEV8` writer - EEV8"]
pub use EEV7_W as EEV8_W;
#[doc = "Field `OCHPEV` reader - OCHPEV"]
pub type OCHPEV_R = crate::BitReader<OCHPEV_A>;
#[doc = "OCHPEV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCHPEV_A {
    #[doc = "0: Rising edge on an on-chip event has no effect"]
    NoEffect = 0,
    #[doc = "1: Rising edge on an on-chip event triggers a burst mode entry"]
    Trigger = 1,
}
impl From<OCHPEV_A> for bool {
    #[inline(always)]
    fn from(variant: OCHPEV_A) -> Self {
        variant as u8 != 0
    }
}
impl OCHPEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCHPEV_A {
        match self.bits {
            false => OCHPEV_A::NoEffect,
            true => OCHPEV_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoEffect`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OCHPEV_A::NoEffect
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OCHPEV_A::Trigger
    }
}
#[doc = "Field `OCHPEV` writer - OCHPEV"]
pub type OCHPEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRGR_SPEC, OCHPEV_A, O>;
impl<'a, const O: u8> OCHPEV_W<'a, O> {
    #[doc = "Rising edge on an on-chip event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OCHPEV_A::NoEffect)
    }
    #[doc = "Rising edge on an on-chip event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(OCHPEV_A::Trigger)
    }
}
impl R {
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    pub fn mstrst(&self) -> MSTRST_R {
        MSTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    pub fn mstrep(&self) -> MSTREP_R {
        MSTREP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    pub fn tarep(&self) -> TAREP_R {
        TAREP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    pub fn tbrep(&self) -> TBREP_R {
        TBREP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    pub fn tcrep(&self) -> TCREP_R {
        TCREP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    pub fn tdrep(&self) -> TDREP_R {
        TDREP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    pub fn terep(&self) -> TEREP_R {
        TEREP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TAEEV7"]
    #[inline(always)]
    pub fn taeev7(&self) -> TAEEV7_R {
        TAEEV7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TDEEV8"]
    #[inline(always)]
    pub fn tdeev8(&self) -> TDEEV8_R {
        TDEEV8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EEV7"]
    #[inline(always)]
    pub fn eev7(&self) -> EEV7_R {
        EEV7_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EEV8"]
    #[inline(always)]
    pub fn eev8(&self) -> EEV8_R {
        EEV8_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    pub fn ochpev(&self) -> OCHPEV_R {
        OCHPEV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    #[must_use]
    pub fn mstrst(&mut self) -> MSTRST_W<1> {
        MSTRST_W::new(self)
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    #[must_use]
    pub fn mstrep(&mut self) -> MSTREP_W<2> {
        MSTREP_W::new(self)
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<3> {
        MSTCMP1_W::new(self)
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<4> {
        MSTCMP2_W::new(self)
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<5> {
        MSTCMP3_W::new(self)
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<6> {
        MSTCMP4_W::new(self)
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    #[must_use]
    pub fn tarst(&mut self) -> TARST_W<7> {
        TARST_W::new(self)
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    #[must_use]
    pub fn tarep(&mut self) -> TAREP_W<8> {
        TAREP_W::new(self)
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp1(&mut self) -> TACMP1_W<9> {
        TACMP1_W::new(self)
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp2(&mut self) -> TACMP2_W<10> {
        TACMP2_W::new(self)
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    #[must_use]
    pub fn tbrst(&mut self) -> TBRST_W<11> {
        TBRST_W::new(self)
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    #[must_use]
    pub fn tbrep(&mut self) -> TBREP_W<12> {
        TBREP_W::new(self)
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<13> {
        TBCMP1_W::new(self)
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<14> {
        TBCMP2_W::new(self)
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    #[must_use]
    pub fn tcrst(&mut self) -> TCRST_W<15> {
        TCRST_W::new(self)
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    #[must_use]
    pub fn tcrep(&mut self) -> TCREP_W<16> {
        TCREP_W::new(self)
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp1(&mut self) -> TCCMP1_W<17> {
        TCCMP1_W::new(self)
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp2(&mut self) -> TCCMP2_W<18> {
        TCCMP2_W::new(self)
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    #[must_use]
    pub fn tdrst(&mut self) -> TDRST_W<19> {
        TDRST_W::new(self)
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    #[must_use]
    pub fn tdrep(&mut self) -> TDREP_W<20> {
        TDREP_W::new(self)
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp1(&mut self) -> TDCMP1_W<21> {
        TDCMP1_W::new(self)
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp2(&mut self) -> TDCMP2_W<22> {
        TDCMP2_W::new(self)
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    #[must_use]
    pub fn terst(&mut self) -> TERST_W<23> {
        TERST_W::new(self)
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    #[must_use]
    pub fn terep(&mut self) -> TEREP_W<24> {
        TEREP_W::new(self)
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp1(&mut self) -> TECMP1_W<25> {
        TECMP1_W::new(self)
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp2(&mut self) -> TECMP2_W<26> {
        TECMP2_W::new(self)
    }
    #[doc = "Bit 27 - TAEEV7"]
    #[inline(always)]
    #[must_use]
    pub fn taeev7(&mut self) -> TAEEV7_W<27> {
        TAEEV7_W::new(self)
    }
    #[doc = "Bit 28 - TDEEV8"]
    #[inline(always)]
    #[must_use]
    pub fn tdeev8(&mut self) -> TDEEV8_W<28> {
        TDEEV8_W::new(self)
    }
    #[doc = "Bit 29 - EEV7"]
    #[inline(always)]
    #[must_use]
    pub fn eev7(&mut self) -> EEV7_W<29> {
        EEV7_W::new(self)
    }
    #[doc = "Bit 30 - EEV8"]
    #[inline(always)]
    #[must_use]
    pub fn eev8(&mut self) -> EEV8_W<30> {
        EEV8_W::new(self)
    }
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    #[must_use]
    pub fn ochpev(&mut self) -> OCHPEV_W<31> {
        OCHPEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMTRGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmtrgr](index.html) module"]
pub struct BMTRGR_SPEC;
impl crate::RegisterSpec for BMTRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmtrgr::R](R) reader structure"]
impl crate::Readable for BMTRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmtrgr::W](W) writer structure"]
impl crate::Writable for BMTRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMTRGR to value 0"]
impl crate::Resettable for BMTRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
