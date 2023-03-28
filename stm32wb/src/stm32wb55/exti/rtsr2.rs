#[doc = "Register `RTSR2` reader"]
pub struct R(crate::R<RTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTSR2` writer"]
pub struct W(crate::W<RTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR2_SPEC>;
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
impl From<crate::W<RTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT33` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT33_R = crate::BitReader<bool>;
#[doc = "Field `RT33` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT33_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, bool, O>;
#[doc = "Field `RT40_41` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT40_41_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RT40_41` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT40_41_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTSR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt33(&self) -> RT33_R {
        RT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt40_41(&self) -> RT40_41_R {
        RT40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt33(&mut self) -> RT33_W<1> {
        RT33_W::new(self)
    }
    #[doc = "Bits 8:9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt40_41(&mut self) -> RT40_41_W<8> {
        RT40_41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr2](index.html) module"]
pub struct RTSR2_SPEC;
impl crate::RegisterSpec for RTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtsr2::R](R) reader structure"]
impl crate::Readable for RTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtsr2::W](W) writer structure"]
impl crate::Writable for RTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for RTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
