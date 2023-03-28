#[doc = "Register `FMC_PCR` reader"]
pub struct R(crate::R<FMC_PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_PCR` writer"]
pub struct W(crate::W<FMC_PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_PCR_SPEC>;
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
impl From<crate::W<FMC_PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub type PWAITEN_R = crate::BitReader<bool>;
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub type PWAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `PBKEN` reader - PBKEN"]
pub type PBKEN_R = crate::BitReader<bool>;
#[doc = "Field `PBKEN` writer - PBKEN"]
pub type PBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `PWID` reader - PWID"]
pub type PWID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWID` writer - PWID"]
pub type PWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type ECCEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type ECCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `ECCALG` reader - ECCALG"]
pub type ECCALG_R = crate::BitReader<bool>;
#[doc = "Field `ECCALG` writer - ECCALG"]
pub type ECCALG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `TCLR` reader - TCLR"]
pub type TCLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLR` writer - TCLR"]
pub type TCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TAR` reader - TAR"]
pub type TAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAR` writer - TAR"]
pub type TAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ECCSS` reader - ECCSS"]
pub type ECCSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCSS` writer - ECCSS"]
pub type ECCSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TCEH` reader - TCEH"]
pub type TCEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCEH` writer - TCEH"]
pub type TCEH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_PCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BCHECC` reader - BCHECC"]
pub type BCHECC_R = crate::BitReader<bool>;
#[doc = "Field `BCHECC` writer - BCHECC"]
pub type BCHECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
#[doc = "Field `WEN` reader - WEN"]
pub type WEN_R = crate::BitReader<bool>;
#[doc = "Field `WEN` writer - WEN"]
pub type WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_PCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - ECCALG"]
    #[inline(always)]
    pub fn eccalg(&self) -> ECCALG_R {
        ECCALG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECCSS"]
    #[inline(always)]
    pub fn eccss(&self) -> ECCSS_R {
        ECCSS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - TCEH"]
    #[inline(always)]
    pub fn tceh(&self) -> TCEH_R {
        TCEH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - BCHECC"]
    #[inline(always)]
    pub fn bchecc(&self) -> BCHECC_R {
        BCHECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WEN"]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<1> {
        PWAITEN_W::new(self)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<2> {
        PBKEN_W::new(self)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<4> {
        PWID_W::new(self)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<6> {
        ECCEN_W::new(self)
    }
    #[doc = "Bit 8 - ECCALG"]
    #[inline(always)]
    #[must_use]
    pub fn eccalg(&mut self) -> ECCALG_W<8> {
        ECCALG_W::new(self)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<9> {
        TCLR_W::new(self)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<13> {
        TAR_W::new(self)
    }
    #[doc = "Bits 17:19 - ECCSS"]
    #[inline(always)]
    #[must_use]
    pub fn eccss(&mut self) -> ECCSS_W<17> {
        ECCSS_W::new(self)
    }
    #[doc = "Bits 20:23 - TCEH"]
    #[inline(always)]
    #[must_use]
    pub fn tceh(&mut self) -> TCEH_W<20> {
        TCEH_W::new(self)
    }
    #[doc = "Bit 24 - BCHECC"]
    #[inline(always)]
    #[must_use]
    pub fn bchecc(&mut self) -> BCHECC_W<24> {
        BCHECC_W::new(self)
    }
    #[doc = "Bit 25 - WEN"]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WEN_W<25> {
        WEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND Flash Programmable control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pcr](index.html) module"]
pub struct FMC_PCR_SPEC;
impl crate::RegisterSpec for FMC_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_pcr::R](R) reader structure"]
impl crate::Readable for FMC_PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_pcr::W](W) writer structure"]
impl crate::Writable for FMC_PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_PCR to value 0x0007_fe08"]
impl crate::Resettable for FMC_PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_fe08;
}
