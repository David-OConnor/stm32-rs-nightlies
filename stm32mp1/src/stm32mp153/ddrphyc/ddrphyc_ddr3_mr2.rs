#[doc = "Register `DDRPHYC_DDR3_MR2` reader"]
pub struct R(crate::R<DDRPHYC_DDR3_MR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DDR3_MR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DDR3_MR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DDR3_MR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DDR3_MR2` writer"]
pub struct W(crate::W<DDRPHYC_DDR3_MR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DDR3_MR2_SPEC>;
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
impl From<crate::W<DDRPHYC_DDR3_MR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DDR3_MR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PASR` reader - PASR"]
pub type PASR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PASR` writer - PASR"]
pub type PASR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DDRPHYC_DDR3_MR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `CWL` reader - CWL"]
pub type CWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CWL` writer - CWL"]
pub type CWL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DDRPHYC_DDR3_MR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ASR` reader - ASR"]
pub type ASR_R = crate::BitReader<bool>;
#[doc = "Field `ASR` writer - ASR"]
pub type ASR_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR2_SPEC, bool, O>;
#[doc = "Field `SRT` reader - SRT"]
pub type SRT_R = crate::BitReader<bool>;
#[doc = "Field `SRT` writer - SRT"]
pub type SRT_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR2_SPEC, bool, O>;
#[doc = "Field `RTTWR` reader - RTTWR"]
pub type RTTWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTTWR` writer - RTTWR"]
pub type RTTWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, DDRPHYC_DDR3_MR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2 - PASR"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - CWL"]
    #[inline(always)]
    pub fn cwl(&self) -> CWL_R {
        CWL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - ASR"]
    #[inline(always)]
    pub fn asr(&self) -> ASR_R {
        ASR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRT"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:10 - RTTWR"]
    #[inline(always)]
    pub fn rttwr(&self) -> RTTWR_R {
        RTTWR_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PASR"]
    #[inline(always)]
    #[must_use]
    pub fn pasr(&mut self) -> PASR_W<0> {
        PASR_W::new(self)
    }
    #[doc = "Bits 3:5 - CWL"]
    #[inline(always)]
    #[must_use]
    pub fn cwl(&mut self) -> CWL_W<3> {
        CWL_W::new(self)
    }
    #[doc = "Bit 6 - ASR"]
    #[inline(always)]
    #[must_use]
    pub fn asr(&mut self) -> ASR_W<6> {
        ASR_W::new(self)
    }
    #[doc = "Bit 7 - SRT"]
    #[inline(always)]
    #[must_use]
    pub fn srt(&mut self) -> SRT_W<7> {
        SRT_W::new(self)
    }
    #[doc = "Bits 9:10 - RTTWR"]
    #[inline(always)]
    #[must_use]
    pub fn rttwr(&mut self) -> RTTWR_W<9> {
        RTTWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC MR2 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr2](index.html) module"]
pub struct DDRPHYC_DDR3_MR2_SPEC;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ddrphyc_ddr3_mr2::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr2::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR2 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
