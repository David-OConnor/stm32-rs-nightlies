#[doc = "Register `BSEC_JTAGOUT` reader"]
pub struct R(crate::R<BSEC_JTAGOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_JTAGOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_JTAGOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_JTAGOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_JTAGOUT` writer"]
pub struct W(crate::W<BSEC_JTAGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_JTAGOUT_SPEC>;
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
impl From<crate::W<BSEC_JTAGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_JTAGOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSEC_JTAGOUT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC JTAG output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_jtagout](index.html) module"]
pub struct BSEC_JTAGOUT_SPEC;
impl crate::RegisterSpec for BSEC_JTAGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_jtagout::R](R) reader structure"]
impl crate::Readable for BSEC_JTAGOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_jtagout::W](W) writer structure"]
impl crate::Writable for BSEC_JTAGOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_JTAGOUT to value 0"]
impl crate::Resettable for BSEC_JTAGOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
