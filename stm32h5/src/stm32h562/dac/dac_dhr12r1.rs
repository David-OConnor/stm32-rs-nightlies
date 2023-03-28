#[doc = "Register `DAC_DHR12R1` reader"]
pub struct R(crate::R<DAC_DHR12R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DHR12R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DHR12R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DHR12R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DHR12R1` writer"]
pub struct W(crate::W<DAC_DHR12R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DHR12R1_SPEC>;
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
impl From<crate::W<DAC_DHR12R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DHR12R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
pub type DACC1DHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
pub type DACC1DHR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_DHR12R1_SPEC, u16, u16, 12, O>;
#[doc = "Field `DACC1DHRB` reader - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode."]
pub type DACC1DHRB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC1DHRB` writer - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode."]
pub type DACC1DHRB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_DHR12R1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode."]
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1."]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<0> {
        DACC1DHR_W::new(self)
    }
    #[doc = "Bits 16:27 - DAC channel1 12-bit right-aligned data B These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC operates in Double data mode."]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<16> {
        DACC1DHRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC channel1 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12r1](index.html) module"]
pub struct DAC_DHR12R1_SPEC;
impl crate::RegisterSpec for DAC_DHR12R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dhr12r1::R](R) reader structure"]
impl crate::Readable for DAC_DHR12R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_dhr12r1::W](W) writer structure"]
impl crate::Writable for DAC_DHR12R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_DHR12R1 to value 0"]
impl crate::Resettable for DAC_DHR12R1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
