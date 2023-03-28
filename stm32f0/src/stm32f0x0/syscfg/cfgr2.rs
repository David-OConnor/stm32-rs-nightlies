#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M0 LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_R = crate::BitReader<LOCKUP_LOCK_A>;
#[doc = "Cortex-M0 LOCKUP bit enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUP_LOCK_A {
    #[doc = "0: Cortex-M0 LOCKUP output disconnected from TIM1/15/16/17 Break input"]
    Disconnected = 0,
    #[doc = "1: Cortex-M0 LOCKUP output connected to TIM1/15/16/17 Break input"]
    Connected = 1,
}
impl From<LOCKUP_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKUP_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_LOCK_A {
        match self.bits {
            false => LOCKUP_LOCK_A::Disconnected,
            true => LOCKUP_LOCK_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == LOCKUP_LOCK_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == LOCKUP_LOCK_A::Connected
    }
}
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M0 LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, LOCKUP_LOCK_A, O>;
impl<'a, const O: u8> LOCKUP_LOCK_W<'a, O> {
    #[doc = "Cortex-M0 LOCKUP output disconnected from TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(LOCKUP_LOCK_A::Disconnected)
    }
    #[doc = "Cortex-M0 LOCKUP output connected to TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(LOCKUP_LOCK_A::Connected)
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_R = crate::BitReader<SRAM_PARITY_LOCK_A>;
#[doc = "SRAM parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PARITY_LOCK_A {
    #[doc = "0: SRAM parity error disconnected from TIM1/15/16/17 Break input"]
    Disconnected = 0,
    #[doc = "1: SRAM parity error connected to TIM1/15/16/17 Break input"]
    Connected = 1,
}
impl From<SRAM_PARITY_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PARITY_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_PARITY_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PARITY_LOCK_A {
        match self.bits {
            false => SRAM_PARITY_LOCK_A::Disconnected,
            true => SRAM_PARITY_LOCK_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == SRAM_PARITY_LOCK_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == SRAM_PARITY_LOCK_A::Connected
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR2_SPEC, SRAM_PARITY_LOCK_A, O>;
impl<'a, const O: u8> SRAM_PARITY_LOCK_W<'a, O> {
    #[doc = "SRAM parity error disconnected from TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(SRAM_PARITY_LOCK_A::Disconnected)
    }
    #[doc = "SRAM parity error connected to TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(SRAM_PARITY_LOCK_A::Connected)
    }
}
#[doc = "Field `SRAM_PEF` reader - SRAM parity flag"]
pub type SRAM_PEF_R = crate::BitReader<SRAM_PEFR_A>;
#[doc = "SRAM parity flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PEFR_A {
    #[doc = "0: No SRAM parity error detected"]
    NoParityError = 0,
    #[doc = "1: SRAM parity error detected"]
    ParityErrorDetected = 1,
}
impl From<SRAM_PEFR_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PEFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_PEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PEFR_A {
        match self.bits {
            false => SRAM_PEFR_A::NoParityError,
            true => SRAM_PEFR_A::ParityErrorDetected,
        }
    }
    #[doc = "Checks if the value of the field is `NoParityError`"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        *self == SRAM_PEFR_A::NoParityError
    }
    #[doc = "Checks if the value of the field is `ParityErrorDetected`"]
    #[inline(always)]
    pub fn is_parity_error_detected(&self) -> bool {
        *self == SRAM_PEFR_A::ParityErrorDetected
    }
}
#[doc = "SRAM parity flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PEFW_AW {
    #[doc = "1: Clear SRAM parity error flag"]
    Clear = 1,
}
impl From<SRAM_PEFW_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PEFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PEF` writer - SRAM parity flag"]
pub type SRAM_PEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SRAM_PEFW_AW, O>;
impl<'a, const O: u8> SRAM_PEF_W<'a, O> {
    #[doc = "Clear SRAM parity error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SRAM_PEFW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<0> {
        LOCKUP_LOCK_W::new(self)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W<1> {
        SRAM_PARITY_LOCK_W::new(self)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W<8> {
        SRAM_PEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
