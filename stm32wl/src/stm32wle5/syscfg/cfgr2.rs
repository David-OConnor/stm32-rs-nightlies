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
#[doc = "Field `CLL` reader - CPU1 LOCKUP (Hardfault) output enable bit"]
pub type CLL_R = crate::BitReader<CLLR_A>;
#[doc = "CPU1 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLLR_A {
    #[doc = "0: CPU LOCKUP output disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: CPU LOCKUP output connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<CLLR_A> for bool {
    #[inline(always)]
    fn from(variant: CLLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLLR_A {
        match self.bits {
            false => CLLR_A::Disconnected,
            true => CLLR_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CLLR_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CLLR_A::Connected
    }
}
#[doc = "CPU1 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLLW_AW {
    #[doc = "1: Connect CPU LOCKUP output to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<CLLW_AW> for bool {
    #[inline(always)]
    fn from(variant: CLLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` writer - CPU1 LOCKUP (Hardfault) output enable bit"]
pub type CLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, CLLW_AW, O>;
impl<'a, const O: u8> CLL_W<'a, O> {
    #[doc = "Connect CPU LOCKUP output to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(CLLW_AW::Connect)
    }
}
#[doc = "Field `SPL` reader - SRAM2 parity lock bit"]
pub type SPL_R = crate::BitReader<SPLR_A>;
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLR_A {
    #[doc = "0: SRAM2 parity error signal disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: SRAM2 parity error signal connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<SPLR_A> for bool {
    #[inline(always)]
    fn from(variant: SPLR_A) -> Self {
        variant as u8 != 0
    }
}
impl SPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLR_A {
        match self.bits {
            false => SPLR_A::Disconnected,
            true => SPLR_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == SPLR_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == SPLR_A::Connected
    }
}
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLW_AW {
    #[doc = "1: Connect SRAM2 parity error signal to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<SPLW_AW> for bool {
    #[inline(always)]
    fn from(variant: SPLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub type SPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SPLW_AW, O>;
impl<'a, const O: u8> SPL_W<'a, O> {
    #[doc = "Connect SRAM2 parity error signal to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(SPLW_AW::Connect)
    }
}
#[doc = "Field `PVDL` reader - PVD lock enable bit"]
pub type PVDL_R = crate::BitReader<PVDLR_A>;
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDLR_A {
    #[doc = "0: PVD interrupt disconnected from TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    Disconnected = 0,
    #[doc = "1: PVD interrupt connected to TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits are read only"]
    Connected = 1,
}
impl From<PVDLR_A> for bool {
    #[inline(always)]
    fn from(variant: PVDLR_A) -> Self {
        variant as u8 != 0
    }
}
impl PVDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDLR_A {
        match self.bits {
            false => PVDLR_A::Disconnected,
            true => PVDLR_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PVDLR_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == PVDLR_A::Connected
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDLW_AW {
    #[doc = "1: Connect PVD interretup to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<PVDLW_AW> for bool {
    #[inline(always)]
    fn from(variant: PVDLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, PVDLW_AW, O>;
impl<'a, const O: u8> PVDL_W<'a, O> {
    #[doc = "Connect PVD interretup to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(PVDLW_AW::Connect)
    }
}
#[doc = "Field `ECCL` reader - ECC Lock"]
pub type ECCL_R = crate::BitReader<ECCLR_A>;
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCLR_A {
    #[doc = "0: ECC error disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: ECC error connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<ECCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ECCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCLR_A {
        match self.bits {
            false => ECCLR_A::Disconnected,
            true => ECCLR_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == ECCLR_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == ECCLR_A::Connected
    }
}
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCLW_AW {
    #[doc = "1: Connect ECC error to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<ECCLW_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type ECCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ECCLW_AW, O>;
impl<'a, const O: u8> ECCL_W<'a, O> {
    #[doc = "Connect ECC error to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(ECCLW_AW::Connect)
    }
}
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub type SPF_R = crate::BitReader<SPFR_A>;
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPFR_A {
    #[doc = "0: No SRAM2 parity error detected"]
    Nominal = 0,
    #[doc = "1: SRAM2 parity error detected"]
    Error = 1,
}
impl From<SPFR_A> for bool {
    #[inline(always)]
    fn from(variant: SPFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPFR_A {
        match self.bits {
            false => SPFR_A::Nominal,
            true => SPFR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `Nominal`"]
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        *self == SPFR_A::Nominal
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SPFR_A::Error
    }
}
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPFW_AW {
    #[doc = "1: Clear SRAM2 parity error flag"]
    Clear = 1,
}
impl From<SPFW_AW> for bool {
    #[inline(always)]
    fn from(variant: SPFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub type SPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SPFW_AW, O>;
impl<'a, const O: u8> SPF_W<'a, O> {
    #[doc = "Clear SRAM2 parity error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SPFW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - CPU1 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 LOCKUP (Hardfault) output enable bit"]
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
