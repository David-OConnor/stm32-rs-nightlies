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
#[doc = "Field `SQ10` reader - 10th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 10th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ10` writer - 10th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 10th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ11` reader - 11th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 11th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ11` writer - 11th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 11th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ12` reader - 12th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 12th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ12` writer - 12th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 12th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ13` reader - 13th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 13th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ13` writer - 13th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 13th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ14` reader - 14th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 14th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ14` writer - 14th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 14th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR3_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 10th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 10th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - 11th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 11th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - 12th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 12th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - 13th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 13th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 14th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 14th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 10th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 10th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq10(&mut self) -> SQ10_W<0> {
        SQ10_W::new(self)
    }
    #[doc = "Bits 6:10 - 11th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 11th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq11(&mut self) -> SQ11_W<6> {
        SQ11_W::new(self)
    }
    #[doc = "Bits 12:16 - 12th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 12th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq12(&mut self) -> SQ12_W<12> {
        SQ12_W::new(self)
    }
    #[doc = "Bits 18:22 - 13th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 13th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq13(&mut self) -> SQ13_W<18> {
        SQ13_W::new(self)
    }
    #[doc = "Bits 24:28 - 14th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 14th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq14(&mut self) -> SQ14_W<24> {
        SQ14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC regular sequence register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr3](index.html) module"]
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
