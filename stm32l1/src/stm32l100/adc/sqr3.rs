#[doc = "Register `SQR3` reader"]
pub struct R(crate::R<SQR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR3` writer"]
pub struct W(crate::W<SQR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR3_SPEC>;
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
impl From<crate::W<SQR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ13` reader - 13th conversion in regular sequence"]
pub type SQ13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ13` writer - 13th conversion in regular sequence"]
pub type SQ13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ14` reader - 14th conversion in regular sequence"]
pub type SQ14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ14` writer - 14th conversion in regular sequence"]
pub type SQ14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ15` reader - 15th conversion in regular sequence"]
pub type SQ15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ15` writer - 15th conversion in regular sequence"]
pub type SQ15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ16` reader - 16th conversion in regular sequence"]
pub type SQ16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ16` writer - 16th conversion in regular sequence"]
pub type SQ16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ17` reader - 17th conversion in regular sequence"]
pub type SQ17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ17` writer - 17th conversion in regular sequence"]
pub type SQ17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ18` reader - 18th conversion in regular sequence"]
pub type SQ18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ18` writer - 18th conversion in regular sequence"]
pub type SQ18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 17th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq17(&self) -> SQ17_R {
        SQ17_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 18th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq18(&self) -> SQ18_R {
        SQ18_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq13(&mut self) -> SQ13_W<0> {
        SQ13_W::new(self)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq14(&mut self) -> SQ14_W<5> {
        SQ14_W::new(self)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq15(&mut self) -> SQ15_W<10> {
        SQ15_W::new(self)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq16(&mut self) -> SQ16_W<15> {
        SQ16_W::new(self)
    }
    #[doc = "Bits 20:24 - 17th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq17(&mut self) -> SQ17_W<20> {
        SQ17_W::new(self)
    }
    #[doc = "Bits 25:29 - 18th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq18(&mut self) -> SQ18_W<25> {
        SQ18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr3](index.html) module"]
pub struct SQR3_SPEC;
impl crate::RegisterSpec for SQR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr3::R](R) reader structure"]
impl crate::Readable for SQR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr3::W](W) writer structure"]
impl crate::Writable for SQR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SQR3 to value 0"]
impl crate::Resettable for SQR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
