#[doc = "Register `MTLOMR` reader"]
pub struct R(crate::R<MTLOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLOMR` writer"]
pub struct W(crate::W<MTLOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLOMR_SPEC>;
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
impl From<crate::W<MTLOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTXSTS` reader - Drop Transmit Status"]
pub type DTXSTS_R = crate::BitReader<bool>;
#[doc = "Field `DTXSTS` writer - Drop Transmit Status"]
pub type DTXSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLOMR_SPEC, bool, O>;
#[doc = "Field `CNTPRST` reader - Counters Preset"]
pub type CNTPRST_R = crate::BitReader<bool>;
#[doc = "Field `CNTPRST` writer - Counters Preset"]
pub type CNTPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLOMR_SPEC, bool, O>;
#[doc = "Field `CNTCLR` reader - Counters Reset"]
pub type CNTCLR_R = crate::BitReader<bool>;
#[doc = "Field `CNTCLR` writer - Counters Reset"]
pub type CNTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLOMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Drop Transmit Status"]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Counters Reset"]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Drop Transmit Status"]
    #[inline(always)]
    #[must_use]
    pub fn dtxsts(&mut self) -> DTXSTS_W<1> {
        DTXSTS_W::new(self)
    }
    #[doc = "Bit 8 - Counters Preset"]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<8> {
        CNTPRST_W::new(self)
    }
    #[doc = "Bit 9 - Counters Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cntclr(&mut self) -> CNTCLR_W<9> {
        CNTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlomr](index.html) module"]
pub struct MTLOMR_SPEC;
impl crate::RegisterSpec for MTLOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlomr::R](R) reader structure"]
impl crate::Readable for MTLOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlomr::W](W) writer structure"]
impl crate::Writable for MTLOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLOMR to value 0"]
impl crate::Resettable for MTLOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
