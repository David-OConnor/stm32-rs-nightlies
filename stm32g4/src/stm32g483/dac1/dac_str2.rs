#[doc = "Register `DAC_STR2` reader"]
pub struct R(crate::R<DAC_STR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_STR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_STR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_STR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_STR2` writer"]
pub struct W(crate::W<DAC_STR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_STR2_SPEC>;
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
impl From<crate::W<DAC_STR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_STR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRSTDATA2` reader - DAC Channel 2 Sawtooth reset value"]
pub type STRSTDATA2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STRSTDATA2` writer - DAC Channel 2 Sawtooth reset value"]
pub type STRSTDATA2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_STR2_SPEC, u16, u16, 12, O>;
#[doc = "Field `STDIR2` reader - DAC Channel2 Sawtooth direction setting"]
pub type STDIR2_R = crate::BitReader<bool>;
#[doc = "Field `STDIR2` writer - DAC Channel2 Sawtooth direction setting"]
pub type STDIR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_STR2_SPEC, bool, O>;
#[doc = "Field `STINCDATA2` reader - DAC CH2 Sawtooth increment value (12.4 bit format)"]
pub type STINCDATA2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STINCDATA2` writer - DAC CH2 Sawtooth increment value (12.4 bit format)"]
pub type STINCDATA2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_STR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:11 - DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    pub fn strstdata2(&self) -> STRSTDATA2_R {
        STRSTDATA2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    pub fn stdir2(&self) -> STDIR2_R {
        STDIR2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn stincdata2(&self) -> STINCDATA2_R {
        STINCDATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    #[must_use]
    pub fn strstdata2(&mut self) -> STRSTDATA2_W<0> {
        STRSTDATA2_W::new(self)
    }
    #[doc = "Bit 12 - DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    #[must_use]
    pub fn stdir2(&mut self) -> STDIR2_W<12> {
        STDIR2_W::new(self)
    }
    #[doc = "Bits 16:31 - DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    #[must_use]
    pub fn stincdata2(&mut self) -> STINCDATA2_W<16> {
        STINCDATA2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sawtooth register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_str2](index.html) module"]
pub struct DAC_STR2_SPEC;
impl crate::RegisterSpec for DAC_STR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_str2::R](R) reader structure"]
impl crate::Readable for DAC_STR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_str2::W](W) writer structure"]
impl crate::Writable for DAC_STR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_STR2 to value 0"]
impl crate::Resettable for DAC_STR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
