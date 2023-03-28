#[doc = "Register `DAC_STMODR` reader"]
pub struct R(crate::R<DAC_STMODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_STMODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_STMODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_STMODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_STMODR` writer"]
pub struct W(crate::W<DAC_STMODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_STMODR_SPEC>;
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
impl From<crate::W<DAC_STMODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_STMODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRSTTRIGSEL1` reader - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type STRSTTRIGSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STRSTTRIGSEL1` writer - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type STRSTTRIGSEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_STMODR_SPEC, u8, u8, 4, O>;
#[doc = "Field `STINCTRIGSEL1` reader - DAC Channel 1 Sawtooth Increment trigger selection"]
pub type STINCTRIGSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STINCTRIGSEL1` writer - DAC Channel 1 Sawtooth Increment trigger selection"]
pub type STINCTRIGSEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_STMODR_SPEC, u8, u8, 4, O>;
#[doc = "Field `STRSTTRIGSEL2` reader - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type STRSTTRIGSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STRSTTRIGSEL2` writer - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type STRSTTRIGSEL2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_STMODR_SPEC, u8, u8, 4, O>;
#[doc = "Field `STINCTRIGSEL2` reader - DAC Channel 2 Sawtooth Increment trigger selection"]
pub type STINCTRIGSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STINCTRIGSEL2` writer - DAC Channel 2 Sawtooth Increment trigger selection"]
pub type STINCTRIGSEL2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_STMODR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel1(&self) -> STRSTTRIGSEL1_R {
        STRSTTRIGSEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel1(&self) -> STINCTRIGSEL1_R {
        STINCTRIGSEL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel2(&self) -> STRSTTRIGSEL2_R {
        STRSTTRIGSEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel2(&self) -> STINCTRIGSEL2_R {
        STINCTRIGSEL2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn strsttrigsel1(&mut self) -> STRSTTRIGSEL1_W<0> {
        STRSTTRIGSEL1_W::new(self)
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn stinctrigsel1(&mut self) -> STINCTRIGSEL1_W<8> {
        STINCTRIGSEL1_W::new(self)
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn strsttrigsel2(&mut self) -> STRSTTRIGSEL2_W<16> {
        STRSTTRIGSEL2_W::new(self)
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn stinctrigsel2(&mut self) -> STINCTRIGSEL2_W<24> {
        STINCTRIGSEL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sawtooth Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_stmodr](index.html) module"]
pub struct DAC_STMODR_SPEC;
impl crate::RegisterSpec for DAC_STMODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_stmodr::R](R) reader structure"]
impl crate::Readable for DAC_STMODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_stmodr::W](W) writer structure"]
impl crate::Writable for DAC_STMODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_STMODR to value 0"]
impl crate::Resettable for DAC_STMODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
