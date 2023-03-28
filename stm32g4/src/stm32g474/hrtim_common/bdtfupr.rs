#[doc = "Register `BDTFUPR` reader"]
pub struct R(crate::R<BDTFUPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTFUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTFUPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTFUPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTFUPR` writer"]
pub struct W(crate::W<BDTFUPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTFUPR_SPEC>;
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
impl From<crate::W<BDTFUPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTFUPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMxCR` reader - HRTIM_TIMxCR register update enable"]
pub type TIMX_CR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCR` writer - HRTIM_TIMxCR register update enable"]
pub type TIMX_CR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxICR` reader - HRTIM_TIMxICR register update enable"]
pub type TIMX_ICR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxICR` writer - HRTIM_TIMxICR register update enable"]
pub type TIMX_ICR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxDIER` reader - HRTIM_TIMxDIER register update enable"]
pub type TIMX_DIER_R = crate::BitReader<bool>;
#[doc = "Field `TIMxDIER` writer - HRTIM_TIMxDIER register update enable"]
pub type TIMX_DIER_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCNT` reader - HRTIM_CNTxR register update enable"]
pub type TIMX_CNT_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCNT` writer - HRTIM_CNTxR register update enable"]
pub type TIMX_CNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxPER` reader - HRTIM_PERxR register update enable"]
pub type TIMX_PER_R = crate::BitReader<bool>;
#[doc = "Field `TIMxPER` writer - HRTIM_PERxR register update enable"]
pub type TIMX_PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxREP` reader - HRTIM_REPxR register update enable"]
pub type TIMX_REP_R = crate::BitReader<bool>;
#[doc = "Field `TIMxREP` writer - HRTIM_REPxR register update enable"]
pub type TIMX_REP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCMP1` reader - HRTIM_CMP1xR register update enable"]
pub type TIMX_CMP1_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCMP1` writer - HRTIM_CMP1xR register update enable"]
pub type TIMX_CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCMP2` reader - HRTIM_CMP2xR register update enable"]
pub type TIMX_CMP2_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCMP2` writer - HRTIM_CMP2xR register update enable"]
pub type TIMX_CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCMP3` reader - HRTIM_CMP3xR register update enable"]
pub type TIMX_CMP3_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCMP3` writer - HRTIM_CMP3xR register update enable"]
pub type TIMX_CMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCMP4` reader - HRTIM_CMP4xR register update enable"]
pub type TIMX_CMP4_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCMP4` writer - HRTIM_CMP4xR register update enable"]
pub type TIMX_CMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMx_DTxR` reader - HRTIM_DTxR register update enable"]
pub type TIMX_DTX_R_R = crate::BitReader<bool>;
#[doc = "Field `TIMx_DTxR` writer - HRTIM_DTxR register update enable"]
pub type TIMX_DTX_R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxSET1R` reader - HRTIM_SET1xR register update enable"]
pub type TIMX_SET1R_R = crate::BitReader<bool>;
#[doc = "Field `TIMxSET1R` writer - HRTIM_SET1xR register update enable"]
pub type TIMX_SET1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxRST1R` reader - HRTIM_RST1xR register update enable"]
pub type TIMX_RST1R_R = crate::BitReader<bool>;
#[doc = "Field `TIMxRST1R` writer - HRTIM_RST1xR register update enable"]
pub type TIMX_RST1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxSET2R` reader - HRTIM_SET2xR register update enable"]
pub type TIMX_SET2R_R = crate::BitReader<bool>;
#[doc = "Field `TIMxSET2R` writer - HRTIM_SET2xR register update enable"]
pub type TIMX_SET2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxRST2R` reader - HRTIM_RST2xR register update enable"]
pub type TIMX_RST2R_R = crate::BitReader<bool>;
#[doc = "Field `TIMxRST2R` writer - HRTIM_RST2xR register update enable"]
pub type TIMX_RST2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxEEFR1` reader - HRTIM_EEFxR1 register update enable"]
pub type TIMX_EEFR1_R = crate::BitReader<bool>;
#[doc = "Field `TIMxEEFR1` writer - HRTIM_EEFxR1 register update enable"]
pub type TIMX_EEFR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxEEFR2` reader - HRTIM_EEFxR2 register update enable"]
pub type TIMX_EEFR2_R = crate::BitReader<bool>;
#[doc = "Field `TIMxEEFR2` writer - HRTIM_EEFxR2 register update enable"]
pub type TIMX_EEFR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxRSTR` reader - HRTIM_RSTxR register update enable"]
pub type TIMX_RSTR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxRSTR` writer - HRTIM_RSTxR register update enable"]
pub type TIMX_RSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCHPR` reader - HRTIM_CHPxR register update enable"]
pub type TIMX_CHPR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCHPR` writer - HRTIM_CHPxR register update enable"]
pub type TIMX_CHPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxOUTR` reader - HRTIM_OUTxR register update enable"]
pub type TIMX_OUTR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxOUTR` writer - HRTIM_OUTxR register update enable"]
pub type TIMX_OUTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxFLTR` reader - HRTIM_FLTxR register update enable"]
pub type TIMX_FLTR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxFLTR` writer - HRTIM_FLTxR register update enable"]
pub type TIMX_FLTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCR2` reader - TIMxCR2"]
pub type TIMX_CR2_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCR2` writer - TIMxCR2"]
pub type TIMX_CR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
#[doc = "Field `TIMxEEFR3` reader - TIMxEEFR3"]
pub type TIMX_EEFR3_R = crate::BitReader<bool>;
#[doc = "Field `TIMxEEFR3` writer - TIMxEEFR3"]
pub type TIMX_EEFR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTFUPR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HRTIM_TIMxCR register update enable"]
    #[inline(always)]
    pub fn timx_cr(&self) -> TIMX_CR_R {
        TIMX_CR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HRTIM_TIMxICR register update enable"]
    #[inline(always)]
    pub fn timx_icr(&self) -> TIMX_ICR_R {
        TIMX_ICR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HRTIM_TIMxDIER register update enable"]
    #[inline(always)]
    pub fn timx_dier(&self) -> TIMX_DIER_R {
        TIMX_DIER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HRTIM_CNTxR register update enable"]
    #[inline(always)]
    pub fn timx_cnt(&self) -> TIMX_CNT_R {
        TIMX_CNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRTIM_PERxR register update enable"]
    #[inline(always)]
    pub fn timx_per(&self) -> TIMX_PER_R {
        TIMX_PER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRTIM_REPxR register update enable"]
    #[inline(always)]
    pub fn timx_rep(&self) -> TIMX_REP_R {
        TIMX_REP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HRTIM_CMP1xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp1(&self) -> TIMX_CMP1_R {
        TIMX_CMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HRTIM_CMP2xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp2(&self) -> TIMX_CMP2_R {
        TIMX_CMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HRTIM_CMP3xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp3(&self) -> TIMX_CMP3_R {
        TIMX_CMP3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HRTIM_CMP4xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp4(&self) -> TIMX_CMP4_R {
        TIMX_CMP4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HRTIM_DTxR register update enable"]
    #[inline(always)]
    pub fn timx_dtx_r(&self) -> TIMX_DTX_R_R {
        TIMX_DTX_R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRTIM_SET1xR register update enable"]
    #[inline(always)]
    pub fn timx_set1r(&self) -> TIMX_SET1R_R {
        TIMX_SET1R_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HRTIM_RST1xR register update enable"]
    #[inline(always)]
    pub fn timx_rst1r(&self) -> TIMX_RST1R_R {
        TIMX_RST1R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HRTIM_SET2xR register update enable"]
    #[inline(always)]
    pub fn timx_set2r(&self) -> TIMX_SET2R_R {
        TIMX_SET2R_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HRTIM_RST2xR register update enable"]
    #[inline(always)]
    pub fn timx_rst2r(&self) -> TIMX_RST2R_R {
        TIMX_RST2R_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - HRTIM_EEFxR1 register update enable"]
    #[inline(always)]
    pub fn timx_eefr1(&self) -> TIMX_EEFR1_R {
        TIMX_EEFR1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - HRTIM_EEFxR2 register update enable"]
    #[inline(always)]
    pub fn timx_eefr2(&self) -> TIMX_EEFR2_R {
        TIMX_EEFR2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HRTIM_RSTxR register update enable"]
    #[inline(always)]
    pub fn timx_rstr(&self) -> TIMX_RSTR_R {
        TIMX_RSTR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HRTIM_CHPxR register update enable"]
    #[inline(always)]
    pub fn timx_chpr(&self) -> TIMX_CHPR_R {
        TIMX_CHPR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HRTIM_OUTxR register update enable"]
    #[inline(always)]
    pub fn timx_outr(&self) -> TIMX_OUTR_R {
        TIMX_OUTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HRTIM_FLTxR register update enable"]
    #[inline(always)]
    pub fn timx_fltr(&self) -> TIMX_FLTR_R {
        TIMX_FLTR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TIMxCR2"]
    #[inline(always)]
    pub fn timx_cr2(&self) -> TIMX_CR2_R {
        TIMX_CR2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TIMxEEFR3"]
    #[inline(always)]
    pub fn timx_eefr3(&self) -> TIMX_EEFR3_R {
        TIMX_EEFR3_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HRTIM_TIMxCR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_cr(&mut self) -> TIMX_CR_W<0> {
        TIMX_CR_W::new(self)
    }
    #[doc = "Bit 1 - HRTIM_TIMxICR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_icr(&mut self) -> TIMX_ICR_W<1> {
        TIMX_ICR_W::new(self)
    }
    #[doc = "Bit 2 - HRTIM_TIMxDIER register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_dier(&mut self) -> TIMX_DIER_W<2> {
        TIMX_DIER_W::new(self)
    }
    #[doc = "Bit 3 - HRTIM_CNTxR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_cnt(&mut self) -> TIMX_CNT_W<3> {
        TIMX_CNT_W::new(self)
    }
    #[doc = "Bit 4 - HRTIM_PERxR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_per(&mut self) -> TIMX_PER_W<4> {
        TIMX_PER_W::new(self)
    }
    #[doc = "Bit 5 - HRTIM_REPxR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_rep(&mut self) -> TIMX_REP_W<5> {
        TIMX_REP_W::new(self)
    }
    #[doc = "Bit 6 - HRTIM_CMP1xR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp1(&mut self) -> TIMX_CMP1_W<6> {
        TIMX_CMP1_W::new(self)
    }
    #[doc = "Bit 7 - HRTIM_CMP2xR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp2(&mut self) -> TIMX_CMP2_W<7> {
        TIMX_CMP2_W::new(self)
    }
    #[doc = "Bit 8 - HRTIM_CMP3xR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp3(&mut self) -> TIMX_CMP3_W<8> {
        TIMX_CMP3_W::new(self)
    }
    #[doc = "Bit 9 - HRTIM_CMP4xR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp4(&mut self) -> TIMX_CMP4_W<9> {
        TIMX_CMP4_W::new(self)
    }
    #[doc = "Bit 10 - HRTIM_DTxR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_dtx_r(&mut self) -> TIMX_DTX_R_W<10> {
        TIMX_DTX_R_W::new(self)
    }
    #[doc = "Bit 11 - HRTIM_SET1xR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_set1r(&mut self) -> TIMX_SET1R_W<11> {
        TIMX_SET1R_W::new(self)
    }
    #[doc = "Bit 12 - HRTIM_RST1xR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_rst1r(&mut self) -> TIMX_RST1R_W<12> {
        TIMX_RST1R_W::new(self)
    }
    #[doc = "Bit 13 - HRTIM_SET2xR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_set2r(&mut self) -> TIMX_SET2R_W<13> {
        TIMX_SET2R_W::new(self)
    }
    #[doc = "Bit 14 - HRTIM_RST2xR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_rst2r(&mut self) -> TIMX_RST2R_W<14> {
        TIMX_RST2R_W::new(self)
    }
    #[doc = "Bit 15 - HRTIM_EEFxR1 register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_eefr1(&mut self) -> TIMX_EEFR1_W<15> {
        TIMX_EEFR1_W::new(self)
    }
    #[doc = "Bit 16 - HRTIM_EEFxR2 register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_eefr2(&mut self) -> TIMX_EEFR2_W<16> {
        TIMX_EEFR2_W::new(self)
    }
    #[doc = "Bit 17 - HRTIM_RSTxR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_rstr(&mut self) -> TIMX_RSTR_W<17> {
        TIMX_RSTR_W::new(self)
    }
    #[doc = "Bit 18 - HRTIM_CHPxR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_chpr(&mut self) -> TIMX_CHPR_W<18> {
        TIMX_CHPR_W::new(self)
    }
    #[doc = "Bit 19 - HRTIM_OUTxR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_outr(&mut self) -> TIMX_OUTR_W<19> {
        TIMX_OUTR_W::new(self)
    }
    #[doc = "Bit 20 - HRTIM_FLTxR register update enable"]
    #[inline(always)]
    #[must_use]
    pub fn timx_fltr(&mut self) -> TIMX_FLTR_W<20> {
        TIMX_FLTR_W::new(self)
    }
    #[doc = "Bit 21 - TIMxCR2"]
    #[inline(always)]
    #[must_use]
    pub fn timx_cr2(&mut self) -> TIMX_CR2_W<21> {
        TIMX_CR2_W::new(self)
    }
    #[doc = "Bit 22 - TIMxEEFR3"]
    #[inline(always)]
    #[must_use]
    pub fn timx_eefr3(&mut self) -> TIMX_EEFR3_W<22> {
        TIMX_EEFR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtfupr](index.html) module"]
pub struct BDTFUPR_SPEC;
impl crate::RegisterSpec for BDTFUPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdtfupr::R](R) reader structure"]
impl crate::Readable for BDTFUPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtfupr::W](W) writer structure"]
impl crate::Writable for BDTFUPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDTFUPR to value 0"]
impl crate::Resettable for BDTFUPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}