#[doc = "Register `AWD2TR` reader"]
pub struct R(crate::R<AWD2TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD2TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD2TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD2TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD2TR` writer"]
pub struct W(crate::W<AWD2TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD2TR_SPEC>;
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
impl From<crate::W<AWD2TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD2TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LT2` reader - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359."]
pub type LT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LT2` writer - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359."]
pub type LT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWD2TR_SPEC, u16, u16, 12, O>;
#[doc = "Field `HT2` reader - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359."]
pub type HT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HT2` writer - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359."]
pub type HT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWD2TR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359."]
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359."]
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359."]
    #[inline(always)]
    #[must_use]
    pub fn lt2(&mut self) -> LT2_W<0> {
        LT2_W::new(self)
    }
    #[doc = "Bits 16:27 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on page 359."]
    #[inline(always)]
    #[must_use]
    pub fn ht2(&mut self) -> HT2_W<16> {
        HT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd2tr](index.html) module"]
pub struct AWD2TR_SPEC;
impl crate::RegisterSpec for AWD2TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd2tr::R](R) reader structure"]
impl crate::Readable for AWD2TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd2tr::W](W) writer structure"]
impl crate::Writable for AWD2TR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWD2TR to value 0x0fff_0000"]
impl crate::Resettable for AWD2TR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
