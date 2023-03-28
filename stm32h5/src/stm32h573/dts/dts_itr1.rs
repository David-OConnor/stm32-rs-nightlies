#[doc = "Register `DTS_ITR1` reader"]
pub struct R(crate::R<DTS_ITR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_ITR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_ITR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_ITR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_ITR1` writer"]
pub struct W(crate::W<DTS_ITR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_ITR1_SPEC>;
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
impl From<crate::W<DTS_ITR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_ITR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1_LITTHD` reader - Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
pub type TS1_LITTHD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS1_LITTHD` writer - Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
pub type TS1_LITTHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_ITR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `TS1_HITTHD` reader - High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
pub type TS1_HITTHD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS1_HITTHD` writer - High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
pub type TS1_HITTHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_ITR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
    #[inline(always)]
    pub fn ts1_litthd(&self) -> TS1_LITTHD_R {
        TS1_LITTHD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
    #[inline(always)]
    pub fn ts1_hitthd(&self) -> TS1_HITTHD_R {
        TS1_HITTHD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_litthd(&mut self) -> TS1_LITTHD_W<0> {
        TS1_LITTHD_W::new(self)
    }
    #[doc = "Bits 16:31 - High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
    #[inline(always)]
    #[must_use]
    pub fn ts1_hitthd(&mut self) -> TS1_HITTHD_W<16> {
        TS1_HITTHD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature sensor interrupt threshold register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_itr1](index.html) module"]
pub struct DTS_ITR1_SPEC;
impl crate::RegisterSpec for DTS_ITR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_itr1::R](R) reader structure"]
impl crate::Readable for DTS_ITR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_itr1::W](W) writer structure"]
impl crate::Writable for DTS_ITR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTS_ITR1 to value 0"]
impl crate::Resettable for DTS_ITR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
