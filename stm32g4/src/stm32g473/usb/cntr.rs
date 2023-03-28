#[doc = "Register `CNTR` reader"]
pub struct R(crate::R<CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTR` writer"]
pub struct W(crate::W<CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTR_SPEC>;
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
impl From<crate::W<CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRES` reader - FRES"]
pub type FRES_R = crate::BitReader<bool>;
#[doc = "Field `FRES` writer - FRES"]
pub type FRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `PDWN` reader - PDWN"]
pub type PDWN_R = crate::BitReader<bool>;
#[doc = "Field `PDWN` writer - PDWN"]
pub type PDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `LP_MODE` reader - LP_MODE"]
pub type LP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LP_MODE` writer - LP_MODE"]
pub type LP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `FSUSP` reader - FSUSP"]
pub type FSUSP_R = crate::BitReader<bool>;
#[doc = "Field `FSUSP` writer - FSUSP"]
pub type FSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - RESUME"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - RESUME"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `L1RESUME` reader - L1RESUME"]
pub type L1RESUME_R = crate::BitReader<bool>;
#[doc = "Field `L1RESUME` writer - L1RESUME"]
pub type L1RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `L1REQM` reader - L1REQM"]
pub type L1REQM_R = crate::BitReader<bool>;
#[doc = "Field `L1REQM` writer - L1REQM"]
pub type L1REQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `ESOFM` reader - ESOFM"]
pub type ESOFM_R = crate::BitReader<bool>;
#[doc = "Field `ESOFM` writer - ESOFM"]
pub type ESOFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `SOFM` reader - SOFM"]
pub type SOFM_R = crate::BitReader<bool>;
#[doc = "Field `SOFM` writer - SOFM"]
pub type SOFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `RESETM` reader - RESETM"]
pub type RESETM_R = crate::BitReader<bool>;
#[doc = "Field `RESETM` writer - RESETM"]
pub type RESETM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `SUSPM` reader - SUSPM"]
pub type SUSPM_R = crate::BitReader<bool>;
#[doc = "Field `SUSPM` writer - SUSPM"]
pub type SUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `WKUPM` reader - WKUPM"]
pub type WKUPM_R = crate::BitReader<bool>;
#[doc = "Field `WKUPM` writer - WKUPM"]
pub type WKUPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `ERRM` reader - ERRM"]
pub type ERRM_R = crate::BitReader<bool>;
#[doc = "Field `ERRM` writer - ERRM"]
pub type ERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `PMAOVRM` reader - PMAOVRM"]
pub type PMAOVRM_R = crate::BitReader<bool>;
#[doc = "Field `PMAOVRM` writer - PMAOVRM"]
pub type PMAOVRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
#[doc = "Field `CTRM` reader - CTRM"]
pub type CTRM_R = crate::BitReader<bool>;
#[doc = "Field `CTRM` writer - CTRM"]
pub type CTRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FRES"]
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LP_MODE"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FSUSP"]
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RESUME"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L1RESUME"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - L1REQM"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ESOFM"]
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOFM"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RESETM"]
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSPM"]
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WKUPM"]
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ERRM"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PMAOVRM"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTRM"]
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRES"]
    #[inline(always)]
    #[must_use]
    pub fn fres(&mut self) -> FRES_W<0> {
        FRES_W::new(self)
    }
    #[doc = "Bit 1 - PDWN"]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<1> {
        PDWN_W::new(self)
    }
    #[doc = "Bit 2 - LP_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn lp_mode(&mut self) -> LP_MODE_W<2> {
        LP_MODE_W::new(self)
    }
    #[doc = "Bit 3 - FSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn fsusp(&mut self) -> FSUSP_W<3> {
        FSUSP_W::new(self)
    }
    #[doc = "Bit 4 - RESUME"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<4> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 5 - L1RESUME"]
    #[inline(always)]
    #[must_use]
    pub fn l1resume(&mut self) -> L1RESUME_W<5> {
        L1RESUME_W::new(self)
    }
    #[doc = "Bit 7 - L1REQM"]
    #[inline(always)]
    #[must_use]
    pub fn l1reqm(&mut self) -> L1REQM_W<7> {
        L1REQM_W::new(self)
    }
    #[doc = "Bit 8 - ESOFM"]
    #[inline(always)]
    #[must_use]
    pub fn esofm(&mut self) -> ESOFM_W<8> {
        ESOFM_W::new(self)
    }
    #[doc = "Bit 9 - SOFM"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<9> {
        SOFM_W::new(self)
    }
    #[doc = "Bit 10 - RESETM"]
    #[inline(always)]
    #[must_use]
    pub fn resetm(&mut self) -> RESETM_W<10> {
        RESETM_W::new(self)
    }
    #[doc = "Bit 11 - SUSPM"]
    #[inline(always)]
    #[must_use]
    pub fn suspm(&mut self) -> SUSPM_W<11> {
        SUSPM_W::new(self)
    }
    #[doc = "Bit 12 - WKUPM"]
    #[inline(always)]
    #[must_use]
    pub fn wkupm(&mut self) -> WKUPM_W<12> {
        WKUPM_W::new(self)
    }
    #[doc = "Bit 13 - ERRM"]
    #[inline(always)]
    #[must_use]
    pub fn errm(&mut self) -> ERRM_W<13> {
        ERRM_W::new(self)
    }
    #[doc = "Bit 14 - PMAOVRM"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<14> {
        PMAOVRM_W::new(self)
    }
    #[doc = "Bit 15 - CTRM"]
    #[inline(always)]
    #[must_use]
    pub fn ctrm(&mut self) -> CTRM_W<15> {
        CTRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](index.html) module"]
pub struct CNTR_SPEC;
impl crate::RegisterSpec for CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntr::R](R) reader structure"]
impl crate::Readable for CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntr::W](W) writer structure"]
impl crate::Writable for CNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTR to value 0"]
impl crate::Resettable for CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
