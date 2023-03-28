#[doc = "Register `QUADSPI_CR` reader"]
pub struct R(crate::R<QUADSPI_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_CR` writer"]
pub struct W(crate::W<QUADSPI_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_CR_SPEC>;
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
impl From<crate::W<QUADSPI_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `ABORT` reader - ABORT"]
pub type ABORT_R = crate::BitReader<bool>;
#[doc = "Field `ABORT` writer - ABORT"]
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `TCEN` reader - TCEN"]
pub type TCEN_R = crate::BitReader<bool>;
#[doc = "Field `TCEN` writer - TCEN"]
pub type TCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `SSHIFT` reader - SSHIFT"]
pub type SSHIFT_R = crate::BitReader<bool>;
#[doc = "Field `SSHIFT` writer - SSHIFT"]
pub type SSHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `DFM` reader - DFM"]
pub type DFM_R = crate::BitReader<bool>;
#[doc = "Field `DFM` writer - DFM"]
pub type DFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `FSEL` reader - FSEL"]
pub type FSEL_R = crate::BitReader<bool>;
#[doc = "Field `FSEL` writer - FSEL"]
pub type FSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `FTHRES` reader - FTHRES"]
pub type FTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTHRES` writer - FTHRES"]
pub type FTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `FTIE` reader - FTIE"]
pub type FTIE_R = crate::BitReader<bool>;
#[doc = "Field `FTIE` writer - FTIE"]
pub type FTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `SMIE` reader - SMIE"]
pub type SMIE_R = crate::BitReader<bool>;
#[doc = "Field `SMIE` writer - SMIE"]
pub type SMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `TOIE` reader - TOIE"]
pub type TOIE_R = crate::BitReader<bool>;
#[doc = "Field `TOIE` writer - TOIE"]
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `APMS` reader - APMS"]
pub type APMS_R = crate::BitReader<bool>;
#[doc = "Field `APMS` writer - APMS"]
pub type APMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `PMM` reader - PMM"]
pub type PMM_R = crate::BitReader<bool>;
#[doc = "Field `PMM` writer - PMM"]
pub type PMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CR_SPEC, bool, O>;
#[doc = "Field `PRESCALER` reader - PRESCALER"]
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALER` writer - PRESCALER"]
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ABORT"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCEN"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSHIFT"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DFM"]
    #[inline(always)]
    pub fn dfm(&self) -> DFM_R {
        DFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FSEL"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - FTHRES"]
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FTIE"]
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SMIE"]
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TOIE"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - APMS"]
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PMM"]
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - PRESCALER"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - ABORT"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<1> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 2 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<2> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 3 - TCEN"]
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<3> {
        TCEN_W::new(self)
    }
    #[doc = "Bit 4 - SSHIFT"]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<4> {
        SSHIFT_W::new(self)
    }
    #[doc = "Bit 6 - DFM"]
    #[inline(always)]
    #[must_use]
    pub fn dfm(&mut self) -> DFM_W<6> {
        DFM_W::new(self)
    }
    #[doc = "Bit 7 - FSEL"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<7> {
        FSEL_W::new(self)
    }
    #[doc = "Bits 8:11 - FTHRES"]
    #[inline(always)]
    #[must_use]
    pub fn fthres(&mut self) -> FTHRES_W<8> {
        FTHRES_W::new(self)
    }
    #[doc = "Bit 16 - TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<16> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 17 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<17> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 18 - FTIE"]
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<18> {
        FTIE_W::new(self)
    }
    #[doc = "Bit 19 - SMIE"]
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SMIE_W<19> {
        SMIE_W::new(self)
    }
    #[doc = "Bit 20 - TOIE"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<20> {
        TOIE_W::new(self)
    }
    #[doc = "Bit 22 - APMS"]
    #[inline(always)]
    #[must_use]
    pub fn apms(&mut self) -> APMS_W<22> {
        APMS_W::new(self)
    }
    #[doc = "Bit 23 - PMM"]
    #[inline(always)]
    #[must_use]
    pub fn pmm(&mut self) -> PMM_W<23> {
        PMM_W::new(self)
    }
    #[doc = "Bits 24:31 - PRESCALER"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<24> {
        PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_cr](index.html) module"]
pub struct QUADSPI_CR_SPEC;
impl crate::RegisterSpec for QUADSPI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_cr::R](R) reader structure"]
impl crate::Readable for QUADSPI_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_cr::W](W) writer structure"]
impl crate::Writable for QUADSPI_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QUADSPI_CR to value 0"]
impl crate::Resettable for QUADSPI_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
