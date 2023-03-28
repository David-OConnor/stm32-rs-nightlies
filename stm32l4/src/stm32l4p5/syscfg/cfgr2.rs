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
#[doc = "Cortex-M4 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLL_AW {
    #[doc = "0: Cortex®-M4 LOCKUP output disconnected from TIM1/8/15/16/17 Break inputs"]
    Disconnected = 0,
    #[doc = "1: Cortex®-M4 LOCKUP output connected to TIM1/8/15/16/17 Break inputs"]
    Connected = 1,
}
impl From<CLL_AW> for bool {
    #[inline(always)]
    fn from(variant: CLL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` writer - Cortex-M4 LOCKUP (Hardfault) output enable bit"]
pub type CLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, CLL_AW, O>;
impl<'a, const O: u8> CLL_W<'a, O> {
    #[doc = "Cortex®-M4 LOCKUP output disconnected from TIM1/8/15/16/17 Break inputs"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(CLL_AW::Disconnected)
    }
    #[doc = "Cortex®-M4 LOCKUP output connected to TIM1/8/15/16/17 Break inputs"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(CLL_AW::Connected)
    }
}
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPL_AW {
    #[doc = "0: SRAM2 parity error signal disconnected from TIM1/8/15/16/17 Break inputs"]
    Disconnected = 0,
    #[doc = "1: SRAM2 parity error signal connected to TIM1/8/15/16/17 Break inputs"]
    Connected = 1,
}
impl From<SPL_AW> for bool {
    #[inline(always)]
    fn from(variant: SPL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub type SPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SPL_AW, O>;
impl<'a, const O: u8> SPL_W<'a, O> {
    #[doc = "SRAM2 parity error signal disconnected from TIM1/8/15/16/17 Break inputs"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(SPL_AW::Disconnected)
    }
    #[doc = "SRAM2 parity error signal connected to TIM1/8/15/16/17 Break inputs"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(SPL_AW::Connected)
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDL_AW {
    #[doc = "0: PVD interrupt disconnected from TIM1/8/15/16/17 Break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    Disconnected = 0,
    #[doc = "1: PVD interrupt connected to TIM1/8/15/16/17 Break input, PVDE and PLS\\[2:0\\]
bits are read only"]
    Connected = 1,
}
impl From<PVDL_AW> for bool {
    #[inline(always)]
    fn from(variant: PVDL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, PVDL_AW, O>;
impl<'a, const O: u8> PVDL_W<'a, O> {
    #[doc = "PVD interrupt disconnected from TIM1/8/15/16/17 Break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PVDL_AW::Disconnected)
    }
    #[doc = "PVD interrupt connected to TIM1/8/15/16/17 Break input, PVDE and PLS\\[2:0\\]
bits are read only"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(PVDL_AW::Connected)
    }
}
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCL_AW {
    #[doc = "0: ECC error disconnected from TIM1/8/15/16/17 Break input"]
    Disconnected = 0,
    #[doc = "1: ECC error connected to TIM1/8/15/16/17 Break input"]
    Connected = 1,
}
impl From<ECCL_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type ECCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ECCL_AW, O>;
impl<'a, const O: u8> ECCL_W<'a, O> {
    #[doc = "ECC error disconnected from TIM1/8/15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(ECCL_AW::Disconnected)
    }
    #[doc = "ECC error connected to TIM1/8/15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(ECCL_AW::Connected)
    }
}
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub type SPF_R = crate::BitReader<SPF_A>;
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPF_A {
    #[doc = "0: No SRAM2 parity error detected"]
    Cleared = 0,
    #[doc = "1: SRAM2 parity error detected"]
    Set = 1,
}
impl From<SPF_A> for bool {
    #[inline(always)]
    fn from(variant: SPF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPF_A {
        match self.bits {
            false => SPF_A::Cleared,
            true => SPF_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Cleared`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SPF_A::Cleared
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SPF_A::Set
    }
}
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub type SPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SPF_A, O>;
impl<'a, const O: u8> SPF_W<'a, O> {
    #[doc = "No SRAM2 parity error detected"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(SPF_A::Cleared)
    }
    #[doc = "SRAM2 parity error detected"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SPF_A::Set)
    }
}
impl R {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M4 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CLL_W<0> {
        CLL_W::new(self)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<1> {
        SPL_W::new(self)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    #[must_use]
    pub fn eccl(&mut self) -> ECCL_W<3> {
        ECCL_W::new(self)
    }
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    #[must_use]
    pub fn spf(&mut self) -> SPF_W<8> {
        SPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
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
