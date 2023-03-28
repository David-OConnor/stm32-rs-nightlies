#[doc = "Register `SQR2` reader"]
pub struct R(crate::R<SQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR2` writer"]
pub struct W(crate::W<SQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR2_SPEC>;
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
impl From<crate::W<SQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ19` reader - 19th conversion in regular sequence"]
pub type SQ19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ19` writer - 19th conversion in regular sequence"]
pub type SQ19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ20` reader - 20th conversion in regular sequence"]
pub type SQ20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ20` writer - 20th conversion in regular sequence"]
pub type SQ20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ21` reader - 21st conversion in regular sequence"]
pub type SQ21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ21` writer - 21st conversion in regular sequence"]
pub type SQ21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ22` reader - 22nd conversion in regular sequence"]
pub type SQ22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ22` writer - 22nd conversion in regular sequence"]
pub type SQ22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ23` reader - 23rd conversion in regular sequence"]
pub type SQ23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ23` writer - 23rd conversion in regular sequence"]
pub type SQ23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ24` reader - 24th conversion in regular sequence"]
pub type SQ24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ24` writer - 24th conversion in regular sequence"]
pub type SQ24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 19th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq19(&self) -> SQ19_R {
        SQ19_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 20th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq20(&self) -> SQ20_R {
        SQ20_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 21st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq21(&self) -> SQ21_R {
        SQ21_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 22nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq22(&self) -> SQ22_R {
        SQ22_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 23rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq23(&self) -> SQ23_R {
        SQ23_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 24th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq24(&self) -> SQ24_R {
        SQ24_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 19th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq19(&mut self) -> SQ19_W<0> {
        SQ19_W::new(self)
    }
    #[doc = "Bits 5:9 - 20th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq20(&mut self) -> SQ20_W<5> {
        SQ20_W::new(self)
    }
    #[doc = "Bits 10:14 - 21st conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq21(&mut self) -> SQ21_W<10> {
        SQ21_W::new(self)
    }
    #[doc = "Bits 15:19 - 22nd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq22(&mut self) -> SQ22_W<15> {
        SQ22_W::new(self)
    }
    #[doc = "Bits 20:24 - 23rd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq23(&mut self) -> SQ23_W<20> {
        SQ23_W::new(self)
    }
    #[doc = "Bits 25:29 - 24th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq24(&mut self) -> SQ24_W<25> {
        SQ24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr2](index.html) module"]
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr2::R](R) reader structure"]
impl crate::Readable for SQR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr2::W](W) writer structure"]
impl crate::Writable for SQR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
