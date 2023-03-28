#[doc = "Register `DTS_DR` reader"]
pub struct R(crate::R<DTS_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_DR` writer"]
pub struct W(crate::W<DTS_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_DR_SPEC>;
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
impl From<crate::W<DTS_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1_MFREQ` reader - Value of the counter output value for temperature sensor 1"]
pub type TS1_MFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS1_MFREQ` writer - Value of the counter output value for temperature sensor 1"]
pub type TS1_MFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTS_DR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Value of the counter output value for temperature sensor 1"]
    #[inline(always)]
    pub fn ts1_mfreq(&self) -> TS1_MFREQ_R {
        TS1_MFREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of the counter output value for temperature sensor 1"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_mfreq(&mut self) -> TS1_MFREQ_W<0> {
        TS1_MFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature sensor data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_dr](index.html) module"]
pub struct DTS_DR_SPEC;
impl crate::RegisterSpec for DTS_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_dr::R](R) reader structure"]
impl crate::Readable for DTS_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_dr::W](W) writer structure"]
impl crate::Writable for DTS_DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTS_DR to value 0"]
impl crate::Resettable for DTS_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
