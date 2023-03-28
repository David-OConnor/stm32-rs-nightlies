#[doc = "Register `DAC_MCR` reader"]
pub struct R(crate::R<DAC_MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_MCR` writer"]
pub struct W(crate::W<DAC_MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_MCR_SPEC>;
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
impl From<crate::W<DAC_MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE1` reader - MODE1"]
pub type MODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE1` writer - MODE1"]
pub type MODE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_MCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MODE2` reader - MODE2"]
pub type MODE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE2` writer - MODE2"]
pub type MODE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_MCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - MODE1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - MODE2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<0> {
        MODE1_W::new(self)
    }
    #[doc = "Bits 16:18 - MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<16> {
        MODE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_mcr](index.html) module"]
pub struct DAC_MCR_SPEC;
impl crate::RegisterSpec for DAC_MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_mcr::R](R) reader structure"]
impl crate::Readable for DAC_MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_mcr::W](W) writer structure"]
impl crate::Writable for DAC_MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_MCR to value 0"]
impl crate::Resettable for DAC_MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
