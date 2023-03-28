#[doc = "Register `BDTR` reader"]
pub struct R(crate::R<BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTR` writer"]
pub struct W(crate::W<BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTR_SPEC>;
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
impl From<crate::W<BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTG` reader - Dead-time generator setup"]
pub type DTG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTG` writer - Dead-time generator setup"]
pub type DTG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOCK` reader - Lock configuration"]
pub type LOCK_R = crate::FieldReader<u8, LOCK_A>;
#[doc = "Lock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_A {
    #[doc = "0: No write protection"]
    Off = 0,
    #[doc = "1: Level 1 write protection"]
    Level1 = 1,
    #[doc = "2: Level 2 write protection"]
    Level2 = 2,
    #[doc = "3: Level 3 write protection"]
    Level3 = 3,
}
impl From<LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as _
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            0 => LOCK_A::Off,
            1 => LOCK_A::Level1,
            2 => LOCK_A::Level2,
            3 => LOCK_A::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LOCK_A::Off
    }
    #[doc = "Checks if the value of the field is `Level1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == LOCK_A::Level1
    }
    #[doc = "Checks if the value of the field is `Level2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == LOCK_A::Level2
    }
    #[doc = "Checks if the value of the field is `Level3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == LOCK_A::Level3
    }
}
#[doc = "Field `LOCK` writer - Lock configuration"]
pub type LOCK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDTR_SPEC, u8, LOCK_A, 2, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LOCK_A::Off)
    }
    #[doc = "Level 1 write protection"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(LOCK_A::Level1)
    }
    #[doc = "Level 2 write protection"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(LOCK_A::Level2)
    }
    #[doc = "Level 3 write protection"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(LOCK_A::Level3)
    }
}
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode"]
pub type OSSI_R = crate::BitReader<OSSI_A>;
#[doc = "Off-state selection for Idle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI_A {
    #[doc = "0: OC/OCN outputs are disabled when inactive"]
    Disabled = 0,
    #[doc = "1: OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime"]
    Enabled = 1,
}
impl From<OSSI_A> for bool {
    #[inline(always)]
    fn from(variant: OSSI_A) -> Self {
        variant as u8 != 0
    }
}
impl OSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSSI_A {
        match self.bits {
            false => OSSI_A::Disabled,
            true => OSSI_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSI_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSI_A::Enabled
    }
}
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode"]
pub type OSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, OSSI_A, O>;
impl<'a, const O: u8> OSSI_W<'a, O> {
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSI_A::Disabled)
    }
    #[doc = "OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSSI_A::Enabled)
    }
}
#[doc = "Field `OSSR` reader - Off-state selection for Run mode"]
pub type OSSR_R = crate::BitReader<OSSR_A>;
#[doc = "Off-state selection for Run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR_A {
    #[doc = "0: OC/OCN outputs are disabled when inactive"]
    Disabled = 0,
    #[doc = "1: OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1"]
    Enabled = 1,
}
impl From<OSSR_A> for bool {
    #[inline(always)]
    fn from(variant: OSSR_A) -> Self {
        variant as u8 != 0
    }
}
impl OSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSSR_A {
        match self.bits {
            false => OSSR_A::Disabled,
            true => OSSR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSR_A::Enabled
    }
}
#[doc = "Field `OSSR` writer - Off-state selection for Run mode"]
pub type OSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, OSSR_A, O>;
impl<'a, const O: u8> OSSR_W<'a, O> {
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSR_A::Disabled)
    }
    #[doc = "OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSSR_A::Enabled)
    }
}
#[doc = "Field `BKE` reader - Break enable"]
pub type BKE_R = crate::BitReader<BKE_A>;
#[doc = "Break enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE_A {
    #[doc = "0: Break inputs (BRK and CCS clock failure event) disabled"]
    Disabled = 0,
    #[doc = "1: Break inputs (BRK and CCS clock failure event) enabled"]
    Enabled = 1,
}
impl From<BKE_A> for bool {
    #[inline(always)]
    fn from(variant: BKE_A) -> Self {
        variant as u8 != 0
    }
}
impl BKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKE_A {
        match self.bits {
            false => BKE_A::Disabled,
            true => BKE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKE_A::Enabled
    }
}
#[doc = "Field `BKE` writer - Break enable"]
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BKE_A, O>;
impl<'a, const O: u8> BKE_W<'a, O> {
    #[doc = "Break inputs (BRK and CCS clock failure event) disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKE_A::Disabled)
    }
    #[doc = "Break inputs (BRK and CCS clock failure event) enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKE_A::Enabled)
    }
}
#[doc = "Field `BKP` reader - Break polarity"]
pub type BKP_R = crate::BitReader<BKP_A>;
#[doc = "Break polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP_A {
    #[doc = "0: Break input BRK is active low"]
    ActiveLow = 0,
    #[doc = "1: Break input BRK is active high"]
    ActiveHigh = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
impl BKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::ActiveLow,
            true => BKP_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKP_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BKP_A::ActiveHigh
    }
}
#[doc = "Field `BKP` writer - Break polarity"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BKP_A, O>;
impl<'a, const O: u8> BKP_W<'a, O> {
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(BKP_A::ActiveLow)
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(BKP_A::ActiveHigh)
    }
}
#[doc = "Field `AOE` reader - Automatic output enable"]
pub type AOE_R = crate::BitReader<AOE_A>;
#[doc = "Automatic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE_A {
    #[doc = "0: MOE can be set only by software"]
    Disabled = 0,
    #[doc = "1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    Enabled = 1,
}
impl From<AOE_A> for bool {
    #[inline(always)]
    fn from(variant: AOE_A) -> Self {
        variant as u8 != 0
    }
}
impl AOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOE_A {
        match self.bits {
            false => AOE_A::Disabled,
            true => AOE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AOE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AOE_A::Enabled
    }
}
#[doc = "Field `AOE` writer - Automatic output enable"]
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, AOE_A, O>;
impl<'a, const O: u8> AOE_W<'a, O> {
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AOE_A::Disabled)
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AOE_A::Enabled)
    }
}
#[doc = "Field `MOE` reader - Main output enable"]
pub type MOE_R = crate::BitReader<MOE_A>;
#[doc = "Main output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE_A {
    #[doc = "0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit"]
    Disabled = 0,
    #[doc = "1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)"]
    Enabled = 1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        variant as u8 != 0
    }
}
impl MOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::Disabled,
            true => MOE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MOE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE_A::Enabled
    }
}
#[doc = "Field `MOE` writer - Main output enable"]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, MOE_A, O>;
impl<'a, const O: u8> MOE_W<'a, O> {
    #[doc = "OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MOE_A::Disabled)
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MOE_A::Enabled)
    }
}
#[doc = "Field `BKDSRM` reader - Break Disarm"]
pub type BKDSRM_R = crate::BitReader<BKDSRM_A>;
#[doc = "Break Disarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKDSRM_A {
    #[doc = "0: Break input BRK is armed"]
    Armed = 0,
    #[doc = "1: Break input BRK is disarmed"]
    Disarmed = 1,
}
impl From<BKDSRM_A> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM_A) -> Self {
        variant as u8 != 0
    }
}
impl BKDSRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKDSRM_A {
        match self.bits {
            false => BKDSRM_A::Armed,
            true => BKDSRM_A::Disarmed,
        }
    }
    #[doc = "Checks if the value of the field is `Armed`"]
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == BKDSRM_A::Armed
    }
    #[doc = "Checks if the value of the field is `Disarmed`"]
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == BKDSRM_A::Disarmed
    }
}
#[doc = "Field `BKDSRM` writer - Break Disarm"]
pub type BKDSRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BKDSRM_A, O>;
impl<'a, const O: u8> BKDSRM_W<'a, O> {
    #[doc = "Break input BRK is armed"]
    #[inline(always)]
    pub fn armed(self) -> &'a mut W {
        self.variant(BKDSRM_A::Armed)
    }
    #[doc = "Break input BRK is disarmed"]
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut W {
        self.variant(BKDSRM_A::Disarmed)
    }
}
#[doc = "Field `BKBID` reader - Break Bidirectional"]
pub type BKBID_R = crate::BitReader<BKBID_A>;
#[doc = "Break Bidirectional\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKBID_A {
    #[doc = "0: Break input BRK in input mode"]
    Input = 0,
    #[doc = "1: Break input BRK in bidirectional mode"]
    Bidirectional = 1,
}
impl From<BKBID_A> for bool {
    #[inline(always)]
    fn from(variant: BKBID_A) -> Self {
        variant as u8 != 0
    }
}
impl BKBID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKBID_A {
        match self.bits {
            false => BKBID_A::Input,
            true => BKBID_A::Bidirectional,
        }
    }
    #[doc = "Checks if the value of the field is `Input`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BKBID_A::Input
    }
    #[doc = "Checks if the value of the field is `Bidirectional`"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BKBID_A::Bidirectional
    }
}
#[doc = "Field `BKBID` writer - Break Bidirectional"]
pub type BKBID_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BKBID_A, O>;
impl<'a, const O: u8> BKBID_W<'a, O> {
    #[doc = "Break input BRK in input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(BKBID_A::Input)
    }
    #[doc = "Break input BRK in bidirectional mode"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BKBID_A::Bidirectional)
    }
}
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 26 - Break Disarm"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Break Bidirectional"]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<0> {
        DTG_W::new(self)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<10> {
        OSSI_W::new(self)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<11> {
        OSSR_W::new(self)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<12> {
        BKE_W::new(self)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<13> {
        BKP_W::new(self)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<14> {
        AOE_W::new(self)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<15> {
        MOE_W::new(self)
    }
    #[doc = "Bit 26 - Break Disarm"]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<26> {
        BKDSRM_W::new(self)
    }
    #[doc = "Bit 28 - Break Bidirectional"]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<28> {
        BKBID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 break and dead-time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtr](index.html) module"]
pub struct BDTR_SPEC;
impl crate::RegisterSpec for BDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdtr::R](R) reader structure"]
impl crate::Readable for BDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtr::W](W) writer structure"]
impl crate::Writable for BDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
