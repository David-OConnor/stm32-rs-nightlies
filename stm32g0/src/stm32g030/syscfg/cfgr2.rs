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
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M0+ LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M0+ LOCKUP bit enable bit"]
pub type LOCKUP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub type PVD_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub type PVD_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `ECC_LOCK` reader - ECC error lock bit"]
pub type ECC_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `ECC_LOCK` writer - ECC error lock bit"]
pub type ECC_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `SRAM_PEF` reader - SRAM parity error flag"]
pub type SRAM_PEF_R = crate::BitReader<bool>;
#[doc = "Field `SRAM_PEF` writer - SRAM parity error flag"]
pub type SRAM_PEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PA1_CDEN` reader - PA1_CDEN"]
pub type PA1_CDEN_R = crate::BitReader<bool>;
#[doc = "Field `PA1_CDEN` writer - PA1_CDEN"]
pub type PA1_CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PA3_CDEN` reader - PA3_CDEN"]
pub type PA3_CDEN_R = crate::BitReader<bool>;
#[doc = "Field `PA3_CDEN` writer - PA3_CDEN"]
pub type PA3_CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PA5_CDEN` reader - PA5_CDEN"]
pub type PA5_CDEN_R = crate::BitReader<bool>;
#[doc = "Field `PA5_CDEN` writer - PA5_CDEN"]
pub type PA5_CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PA6_CDEN` reader - PA6_CDEN"]
pub type PA6_CDEN_R = crate::BitReader<bool>;
#[doc = "Field `PA6_CDEN` writer - PA6_CDEN"]
pub type PA6_CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PA13_CDEN` reader - PA13_CDEN"]
pub type PA13_CDEN_R = crate::BitReader<bool>;
#[doc = "Field `PA13_CDEN` writer - PA13_CDEN"]
pub type PA13_CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PB0_CDEN` reader - PB0_CDEN"]
pub type PB0_CDEN_R = crate::BitReader<bool>;
#[doc = "Field `PB0_CDEN` writer - PB0_CDEN"]
pub type PB0_CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PB1_CDEN` reader - PB1_CDEN"]
pub type PB1_CDEN_R = crate::BitReader<bool>;
#[doc = "Field `PB1_CDEN` writer - PB1_CDEN"]
pub type PB1_CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PB2_CDEN` reader - PB2_CDEN"]
pub type PB2_CDEN_R = crate::BitReader<bool>;
#[doc = "Field `PB2_CDEN` writer - PB2_CDEN"]
pub type PB2_CDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC error lock bit"]
    #[inline(always)]
    pub fn ecc_lock(&self) -> ECC_LOCK_R {
        ECC_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM parity error flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - PA1_CDEN"]
    #[inline(always)]
    pub fn pa1_cden(&self) -> PA1_CDEN_R {
        PA1_CDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PA3_CDEN"]
    #[inline(always)]
    pub fn pa3_cden(&self) -> PA3_CDEN_R {
        PA3_CDEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PA5_CDEN"]
    #[inline(always)]
    pub fn pa5_cden(&self) -> PA5_CDEN_R {
        PA5_CDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PA6_CDEN"]
    #[inline(always)]
    pub fn pa6_cden(&self) -> PA6_CDEN_R {
        PA6_CDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PA13_CDEN"]
    #[inline(always)]
    pub fn pa13_cden(&self) -> PA13_CDEN_R {
        PA13_CDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PB0_CDEN"]
    #[inline(always)]
    pub fn pb0_cden(&self) -> PB0_CDEN_R {
        PB0_CDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PB1_CDEN"]
    #[inline(always)]
    pub fn pb1_cden(&self) -> PB1_CDEN_R {
        PB1_CDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PB2_CDEN"]
    #[inline(always)]
    pub fn pb2_cden(&self) -> PB2_CDEN_R {
        PB2_CDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
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
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<2> {
        PVD_LOCK_W::new(self)
    }
    #[doc = "Bit 3 - ECC error lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_lock(&mut self) -> ECC_LOCK_W<3> {
        ECC_LOCK_W::new(self)
    }
    #[doc = "Bit 8 - SRAM parity error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W<8> {
        SRAM_PEF_W::new(self)
    }
    #[doc = "Bit 16 - PA1_CDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pa1_cden(&mut self) -> PA1_CDEN_W<16> {
        PA1_CDEN_W::new(self)
    }
    #[doc = "Bit 17 - PA3_CDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pa3_cden(&mut self) -> PA3_CDEN_W<17> {
        PA3_CDEN_W::new(self)
    }
    #[doc = "Bit 18 - PA5_CDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pa5_cden(&mut self) -> PA5_CDEN_W<18> {
        PA5_CDEN_W::new(self)
    }
    #[doc = "Bit 19 - PA6_CDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pa6_cden(&mut self) -> PA6_CDEN_W<19> {
        PA6_CDEN_W::new(self)
    }
    #[doc = "Bit 20 - PA13_CDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pa13_cden(&mut self) -> PA13_CDEN_W<20> {
        PA13_CDEN_W::new(self)
    }
    #[doc = "Bit 21 - PB0_CDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pb0_cden(&mut self) -> PB0_CDEN_W<21> {
        PB0_CDEN_W::new(self)
    }
    #[doc = "Bit 22 - PB1_CDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pb1_cden(&mut self) -> PB1_CDEN_W<22> {
        PB1_CDEN_W::new(self)
    }
    #[doc = "Bit 23 - PB2_CDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pb2_cden(&mut self) -> PB2_CDEN_W<23> {
        PB2_CDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
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
