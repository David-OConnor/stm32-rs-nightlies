#[doc = "Register `PWR_MPUCR` reader"]
pub struct R(crate::R<PWR_MPUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_MPUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_MPUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_MPUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_MPUCR` writer"]
pub struct W(crate::W<PWR_MPUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_MPUCR_SPEC>;
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
impl From<crate::W<PWR_MPUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_MPUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDS` reader - PDDS"]
pub type PDDS_R = crate::BitReader<bool>;
#[doc = "Field `PDDS` writer - PDDS"]
pub type PDDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUCR_SPEC, bool, O>;
#[doc = "Field `CSTBYDIS` reader - CSTBYDIS"]
pub type CSTBYDIS_R = crate::BitReader<bool>;
#[doc = "Field `CSTBYDIS` writer - CSTBYDIS"]
pub type CSTBYDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUCR_SPEC, bool, O>;
#[doc = "Field `STOPF` reader - STOPF"]
pub type STOPF_R = crate::BitReader<bool>;
#[doc = "Field `SBF` reader - SBF"]
pub type SBF_R = crate::BitReader<bool>;
#[doc = "Field `SBFMPU` reader - SBFMPU"]
pub type SBFMPU_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` reader - CSSF"]
pub type CSSF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` writer - CSSF"]
pub type CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUCR_SPEC, bool, O>;
#[doc = "Field `STANDBYWFIL2` reader - STANDBYWFIL2"]
pub type STANDBYWFIL2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PDDS"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - CSTBYDIS"]
    #[inline(always)]
    pub fn cstbydis(&self) -> CSTBYDIS_R {
        CSTBYDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SBF"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SBFMPU"]
    #[inline(always)]
    pub fn sbfmpu(&self) -> SBFMPU_R {
        SBFMPU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CSSF"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - STANDBYWFIL2"]
    #[inline(always)]
    pub fn standbywfil2(&self) -> STANDBYWFIL2_R {
        STANDBYWFIL2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDS"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<0> {
        PDDS_W::new(self)
    }
    #[doc = "Bit 3 - CSTBYDIS"]
    #[inline(always)]
    #[must_use]
    pub fn cstbydis(&mut self) -> CSTBYDIS_W<3> {
        CSTBYDIS_W::new(self)
    }
    #[doc = "Bit 9 - CSSF"]
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<9> {
        CSSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_mpucr](index.html) module"]
pub struct PWR_MPUCR_SPEC;
impl crate::RegisterSpec for PWR_MPUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_mpucr::R](R) reader structure"]
impl crate::Readable for PWR_MPUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_mpucr::W](W) writer structure"]
impl crate::Writable for PWR_MPUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_MPUCR to value 0"]
impl crate::Resettable for PWR_MPUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
