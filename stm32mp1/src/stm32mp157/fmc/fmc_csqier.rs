#[doc = "Register `FMC_CSQIER` reader"]
pub struct R(crate::R<FMC_CSQIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQIER` writer"]
pub struct W(crate::W<FMC_CSQIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQIER_SPEC>;
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
impl From<crate::W<FMC_CSQIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQIER_SPEC, bool, O>;
#[doc = "Field `SCIE` reader - SCIE"]
pub type SCIE_R = crate::BitReader<bool>;
#[doc = "Field `SCIE` writer - SCIE"]
pub type SCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQIER_SPEC, bool, O>;
#[doc = "Field `SEIE` reader - SEIE"]
pub type SEIE_R = crate::BitReader<bool>;
#[doc = "Field `SEIE` writer - SEIE"]
pub type SEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQIER_SPEC, bool, O>;
#[doc = "Field `SUEIE` reader - SUEIE"]
pub type SUEIE_R = crate::BitReader<bool>;
#[doc = "Field `SUEIE` writer - SUEIE"]
pub type SUEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQIER_SPEC, bool, O>;
#[doc = "Field `CMDTCIE` reader - CMDTCIE"]
pub type CMDTCIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDTCIE` writer - CMDTCIE"]
pub type CMDTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCIE"]
    #[inline(always)]
    pub fn scie(&self) -> SCIE_R {
        SCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEIE"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SUEIE"]
    #[inline(always)]
    pub fn sueie(&self) -> SUEIE_R {
        SUEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMDTCIE"]
    #[inline(always)]
    pub fn cmdtcie(&self) -> CMDTCIE_R {
        CMDTCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<0> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 1 - SCIE"]
    #[inline(always)]
    #[must_use]
    pub fn scie(&mut self) -> SCIE_W<1> {
        SCIE_W::new(self)
    }
    #[doc = "Bit 2 - SEIE"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<2> {
        SEIE_W::new(self)
    }
    #[doc = "Bit 3 - SUEIE"]
    #[inline(always)]
    #[must_use]
    pub fn sueie(&mut self) -> SUEIE_W<3> {
        SUEIE_W::new(self)
    }
    #[doc = "Bit 4 - CMDTCIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtcie(&mut self) -> CMDTCIE_W<4> {
        CMDTCIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqier](index.html) module"]
pub struct FMC_CSQIER_SPEC;
impl crate::RegisterSpec for FMC_CSQIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqier::R](R) reader structure"]
impl crate::Readable for FMC_CSQIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqier::W](W) writer structure"]
impl crate::Writable for FMC_CSQIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQIER to value 0"]
impl crate::Resettable for FMC_CSQIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
