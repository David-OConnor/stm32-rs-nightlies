#[doc = "Register `FMC_CSQISR` reader"]
pub struct R(crate::R<FMC_CSQISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQISR` writer"]
pub struct W(crate::W<FMC_CSQISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQISR_SPEC>;
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
impl From<crate::W<FMC_CSQISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCF` reader - TCF"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - TCF"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQISR_SPEC, bool, O>;
#[doc = "Field `SCF` reader - SCF"]
pub type SCF_R = crate::BitReader<bool>;
#[doc = "Field `SCF` writer - SCF"]
pub type SCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQISR_SPEC, bool, O>;
#[doc = "Field `SEF` reader - SEF"]
pub type SEF_R = crate::BitReader<bool>;
#[doc = "Field `SEF` writer - SEF"]
pub type SEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQISR_SPEC, bool, O>;
#[doc = "Field `SUEF` reader - SUEF"]
pub type SUEF_R = crate::BitReader<bool>;
#[doc = "Field `SUEF` writer - SUEF"]
pub type SUEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQISR_SPEC, bool, O>;
#[doc = "Field `CMDTCF` reader - CMDTCF"]
pub type CMDTCF_R = crate::BitReader<bool>;
#[doc = "Field `CMDTCF` writer - CMDTCF"]
pub type CMDTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCF"]
    #[inline(always)]
    pub fn scf(&self) -> SCF_R {
        SCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEF"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SUEF"]
    #[inline(always)]
    pub fn suef(&self) -> SUEF_R {
        SUEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMDTCF"]
    #[inline(always)]
    pub fn cmdtcf(&self) -> CMDTCF_R {
        CMDTCF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCF"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<0> {
        TCF_W::new(self)
    }
    #[doc = "Bit 1 - SCF"]
    #[inline(always)]
    #[must_use]
    pub fn scf(&mut self) -> SCF_W<1> {
        SCF_W::new(self)
    }
    #[doc = "Bit 2 - SEF"]
    #[inline(always)]
    #[must_use]
    pub fn sef(&mut self) -> SEF_W<2> {
        SEF_W::new(self)
    }
    #[doc = "Bit 3 - SUEF"]
    #[inline(always)]
    #[must_use]
    pub fn suef(&mut self) -> SUEF_W<3> {
        SUEF_W::new(self)
    }
    #[doc = "Bit 4 - CMDTCF"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtcf(&mut self) -> CMDTCF_W<4> {
        CMDTCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqisr](index.html) module"]
pub struct FMC_CSQISR_SPEC;
impl crate::RegisterSpec for FMC_CSQISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqisr::R](R) reader structure"]
impl crate::Readable for FMC_CSQISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqisr::W](W) writer structure"]
impl crate::Writable for FMC_CSQISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQISR to value 0"]
impl crate::Resettable for FMC_CSQISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
