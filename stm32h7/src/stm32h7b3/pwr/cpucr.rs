#[doc = "Register `CPUCR` reader"]
pub struct R(crate::R<CPUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUCR` writer"]
pub struct W(crate::W<CPUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUCR_SPEC>;
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
impl From<crate::W<CPUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETDS_CD` reader - RETDS_CD"]
pub type RETDS_CD_R = crate::BitReader<bool>;
#[doc = "Field `RETDS_CD` writer - RETDS_CD"]
pub type RETDS_CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUCR_SPEC, bool, O>;
#[doc = "Field `PDDS_SRD` reader - PDDS_SRD"]
pub type PDDS_SRD_R = crate::BitReader<bool>;
#[doc = "Field `PDDS_SRD` writer - PDDS_SRD"]
pub type PDDS_SRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUCR_SPEC, bool, O>;
#[doc = "Field `STOPF` reader - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
pub type STOPF_R = crate::BitReader<bool>;
#[doc = "Field `SBF` reader - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
pub type SBF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` reader - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` writer - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUCR_SPEC, bool, O>;
#[doc = "Field `RUN_SRD` reader - RUN_SRD"]
pub type RUN_SRD_R = crate::BitReader<bool>;
#[doc = "Field `RUN_SRD` writer - RUN_SRD"]
pub type RUN_SRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RETDS_CD"]
    #[inline(always)]
    pub fn retds_cd(&self) -> RETDS_CD_R {
        RETDS_CD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PDDS_SRD"]
    #[inline(always)]
    pub fn pdds_srd(&self) -> PDDS_SRD_R {
        PDDS_SRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - RUN_SRD"]
    #[inline(always)]
    pub fn run_srd(&self) -> RUN_SRD_R {
        RUN_SRD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RETDS_CD"]
    #[inline(always)]
    #[must_use]
    pub fn retds_cd(&mut self) -> RETDS_CD_W<0> {
        RETDS_CD_W::new(self)
    }
    #[doc = "Bit 2 - PDDS_SRD"]
    #[inline(always)]
    #[must_use]
    pub fn pdds_srd(&mut self) -> PDDS_SRD_W<2> {
        PDDS_SRD_W::new(self)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<9> {
        CSSF_W::new(self)
    }
    #[doc = "Bit 11 - RUN_SRD"]
    #[inline(always)]
    #[must_use]
    pub fn run_srd(&mut self) -> RUN_SRD_W<11> {
        RUN_SRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register allows controlling CPU1 power.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpucr](index.html) module"]
pub struct CPUCR_SPEC;
impl crate::RegisterSpec for CPUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpucr::R](R) reader structure"]
impl crate::Readable for CPUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpucr::W](W) writer structure"]
impl crate::Writable for CPUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUCR to value 0"]
impl crate::Resettable for CPUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
