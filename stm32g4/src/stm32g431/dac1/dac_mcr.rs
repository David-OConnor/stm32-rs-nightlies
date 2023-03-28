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
#[doc = "Field `MODE1` reader - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
pub type MODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE1` writer - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
pub type MODE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_MCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DMADOUBLE1` reader - DAC Channel1 DMA double data mode"]
pub type DMADOUBLE1_R = crate::BitReader<bool>;
#[doc = "Field `DMADOUBLE1` writer - DAC Channel1 DMA double data mode"]
pub type DMADOUBLE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_MCR_SPEC, bool, O>;
#[doc = "Field `SINFORMAT1` reader - Enable signed format for DAC channel1"]
pub type SINFORMAT1_R = crate::BitReader<bool>;
#[doc = "Field `SINFORMAT1` writer - Enable signed format for DAC channel1"]
pub type SINFORMAT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_MCR_SPEC, bool, O>;
#[doc = "Field `HFSEL` reader - High frequency interface mode selection"]
pub type HFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HFSEL` writer - High frequency interface mode selection"]
pub type HFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_MCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE2` reader - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
pub type MODE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE2` writer - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
pub type MODE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_MCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DMADOUBLE2` reader - DAC Channel2 DMA double data mode"]
pub type DMADOUBLE2_R = crate::BitReader<bool>;
#[doc = "Field `DMADOUBLE2` writer - DAC Channel2 DMA double data mode"]
pub type DMADOUBLE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_MCR_SPEC, bool, O>;
#[doc = "Field `SINFORMAT2` reader - Enable signed format for DAC channel2"]
pub type SINFORMAT2_R = crate::BitReader<bool>;
#[doc = "Field `SINFORMAT2` writer - Enable signed format for DAC channel2"]
pub type SINFORMAT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_MCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble1(&self) -> DMADOUBLE1_R {
        DMADOUBLE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    pub fn sinformat1(&self) -> SINFORMAT1_R {
        SINFORMAT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble2(&self) -> DMADOUBLE2_R {
        DMADOUBLE2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    pub fn sinformat2(&self) -> SINFORMAT2_R {
        SINFORMAT2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<0> {
        MODE1_W::new(self)
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmadouble1(&mut self) -> DMADOUBLE1_W<8> {
        DMADOUBLE1_W::new(self)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    #[must_use]
    pub fn sinformat1(&mut self) -> SINFORMAT1_W<9> {
        SINFORMAT1_W::new(self)
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn hfsel(&mut self) -> HFSEL_W<14> {
        HFSEL_W::new(self)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<16> {
        MODE2_W::new(self)
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmadouble2(&mut self) -> DMADOUBLE2_W<24> {
        DMADOUBLE2_W::new(self)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    #[must_use]
    pub fn sinformat2(&mut self) -> SINFORMAT2_W<25> {
        SINFORMAT2_W::new(self)
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
