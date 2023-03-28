#[doc = "Register `AWD1TR` reader"]
pub struct R(crate::R<AWD1TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD1TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD1TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD1TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD1TR` writer"]
pub struct W(crate::W<AWD1TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD1TR_SPEC>;
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
impl From<crate::W<AWD1TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD1TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LT1` reader - ADC analog watchdog 1 threshold low"]
pub type LT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LT1` writer - ADC analog watchdog 1 threshold low"]
pub type LT1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWD1TR_SPEC, u16, u16, 12, O>;
#[doc = "Field `HT1` reader - ADC analog watchdog 1 threshold high"]
pub type HT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HT1` writer - ADC analog watchdog 1 threshold high"]
pub type HT1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWD1TR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADC analog watchdog 1 threshold high"]
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    #[must_use]
    pub fn lt1(&mut self) -> LT1_W<0> {
        LT1_W::new(self)
    }
    #[doc = "Bits 16:27 - ADC analog watchdog 1 threshold high"]
    #[inline(always)]
    #[must_use]
    pub fn ht1(&mut self) -> HT1_W<16> {
        HT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd1tr](index.html) module"]
pub struct AWD1TR_SPEC;
impl crate::RegisterSpec for AWD1TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd1tr::R](R) reader structure"]
impl crate::Readable for AWD1TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd1tr::W](W) writer structure"]
impl crate::Writable for AWD1TR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWD1TR to value 0x0fff_0000"]
impl crate::Resettable for AWD1TR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
