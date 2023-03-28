#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDATA_0_6` reader - Regular Data converted 0_6"]
pub type RDATA_0_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDATA_0_6` writer - Regular Data converted 0_6"]
pub type RDATA_0_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 6, O>;
#[doc = "Field `RDATA_7_15` reader - 15"]
pub type RDATA_7_15_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:5 - Regular Data converted 0_6"]
    #[inline(always)]
    pub fn rdata_0_6(&self) -> RDATA_0_6_R {
        RDATA_0_6_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 7:15 - 15"]
    #[inline(always)]
    pub fn rdata_7_15(&self) -> RDATA_7_15_R {
        RDATA_7_15_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Regular Data converted 0_6"]
    #[inline(always)]
    #[must_use]
    pub fn rdata_0_6(&mut self) -> RDATA_0_6_W<0> {
        RDATA_0_6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC group regular conversion data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
