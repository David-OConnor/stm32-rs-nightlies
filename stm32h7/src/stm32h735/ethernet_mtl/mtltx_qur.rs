#[doc = "Register `MTLTxQUR` reader"]
pub struct R(crate::R<MTLTX_QUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTX_QUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTX_QUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTX_QUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLTxQUR` writer"]
pub struct W(crate::W<MTLTX_QUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLTX_QUR_SPEC>;
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
impl From<crate::W<MTLTX_QUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLTX_QUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UFFRMCNT` reader - Underflow Packet Counter"]
pub type UFFRMCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UFFRMCNT` writer - Underflow Packet Counter"]
pub type UFFRMCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLTX_QUR_SPEC, u16, u16, 11, O>;
#[doc = "Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter"]
pub type UFCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `UFCNTOVF` writer - Overflow Bit for Underflow Packet Counter"]
pub type UFCNTOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLTX_QUR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    #[must_use]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W<0> {
        UFFRMCNT_W::new(self)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W<11> {
        UFCNTOVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx queue underflow register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qur](index.html) module"]
pub struct MTLTX_QUR_SPEC;
impl crate::RegisterSpec for MTLTX_QUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtltx_qur::R](R) reader structure"]
impl crate::Readable for MTLTX_QUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtltx_qur::W](W) writer structure"]
impl crate::Writable for MTLTX_QUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLTxQUR to value 0"]
impl crate::Resettable for MTLTX_QUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
