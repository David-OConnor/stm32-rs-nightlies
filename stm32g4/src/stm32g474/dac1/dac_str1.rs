#[doc = "Register `DAC_STR1` reader"]
pub struct R(crate::R<DAC_STR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_STR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_STR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_STR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_STR1` writer"]
pub struct W(crate::W<DAC_STR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_STR1_SPEC>;
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
impl From<crate::W<DAC_STR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_STR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRSTDATA1` reader - DAC Channel 1 Sawtooth reset value"]
pub type STRSTDATA1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STRSTDATA1` writer - DAC Channel 1 Sawtooth reset value"]
pub type STRSTDATA1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_STR1_SPEC, u16, u16, 12, O>;
#[doc = "Field `STDIR1` reader - DAC Channel1 Sawtooth direction setting"]
pub type STDIR1_R = crate::BitReader<bool>;
#[doc = "Field `STDIR1` writer - DAC Channel1 Sawtooth direction setting"]
pub type STDIR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_STR1_SPEC, bool, O>;
#[doc = "Field `STINCDATA1` reader - DAC CH1 Sawtooth increment value (12.4 bit format)"]
pub type STINCDATA1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STINCDATA1` writer - DAC CH1 Sawtooth increment value (12.4 bit format)"]
pub type STINCDATA1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_STR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata1(&self) -> STRSTDATA1_R {
        STRSTDATA1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir1(&self) -> STDIR1_R {
        STDIR1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata1(&self) -> STINCDATA1_R {
        STINCDATA1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    #[must_use]
    pub fn strstdata1(&mut self) -> STRSTDATA1_W<0> {
        STRSTDATA1_W::new(self)
    }
    #[doc = "Bit 12 - DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    #[must_use]
    pub fn stdir1(&mut self) -> STDIR1_W<12> {
        STDIR1_W::new(self)
    }
    #[doc = "Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    #[must_use]
    pub fn stincdata1(&mut self) -> STINCDATA1_W<16> {
        STINCDATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sawtooth register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_str1](index.html) module"]
pub struct DAC_STR1_SPEC;
impl crate::RegisterSpec for DAC_STR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_str1::R](R) reader structure"]
impl crate::Readable for DAC_STR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_str1::W](W) writer structure"]
impl crate::Writable for DAC_STR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_STR1 to value 0"]
impl crate::Resettable for DAC_STR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
