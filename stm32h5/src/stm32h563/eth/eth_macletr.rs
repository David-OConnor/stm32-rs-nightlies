#[doc = "Register `ETH_MACLETR` reader"]
pub struct R(crate::R<ETH_MACLETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACLETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACLETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACLETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACLETR` writer"]
pub struct W(crate::W<ETH_MACLETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACLETR_SPEC>;
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
impl From<crate::W<ETH_MACLETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACLETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPIET` reader - LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames. This field is valid and used only when LPITE and LPITXA are set to 1. Bits \\[2:0\\]
are read-only so that the granularity of this timer is in steps of 8 micro-seconds."]
pub type LPIET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LPIET` writer - LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames. This field is valid and used only when LPITE and LPITXA are set to 1. Bits \\[2:0\\]
are read-only so that the granularity of this timer is in steps of 8 micro-seconds."]
pub type LPIET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLETR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames. This field is valid and used only when LPITE and LPITXA are set to 1. Bits \\[2:0\\]
are read-only so that the granularity of this timer is in steps of 8 micro-seconds."]
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames. This field is valid and used only when LPITE and LPITXA are set to 1. Bits \\[2:0\\]
are read-only so that the granularity of this timer is in steps of 8 micro-seconds."]
    #[inline(always)]
    #[must_use]
    pub fn lpiet(&mut self) -> LPIET_W<0> {
        LPIET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI entry timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macletr](index.html) module"]
pub struct ETH_MACLETR_SPEC;
impl crate::RegisterSpec for ETH_MACLETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macletr::R](R) reader structure"]
impl crate::Readable for ETH_MACLETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macletr::W](W) writer structure"]
impl crate::Writable for ETH_MACLETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACLETR to value 0"]
impl crate::Resettable for ETH_MACLETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
