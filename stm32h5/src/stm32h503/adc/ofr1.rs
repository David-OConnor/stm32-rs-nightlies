#[doc = "Register `OFR1` reader"]
pub struct R(crate::R<OFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR1` writer"]
pub struct W(crate::W<OFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR1_SPEC>;
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
impl From<crate::W<OFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - Data offset y for the channel programmed into bits OFFSET_CH\\[4:0\\]
These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\\[4:0\\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\\[4:0\\]
= 4 and OFFSET2_CH\\[4:0\\]
= 4, this is OFFSET1\\[11:0\\]
which is subtracted when converting channel 4."]
pub type OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET` writer - Data offset y for the channel programmed into bits OFFSET_CH\\[4:0\\]
These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\\[4:0\\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\\[4:0\\]
= 4 and OFFSET2_CH\\[4:0\\]
= 4, this is OFFSET1\\[11:0\\]
which is subtracted when converting channel 4."]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR1_SPEC, u16, u16, 12, O>;
#[doc = "Field `OFFSETPOS` reader - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OFFSETPOS_R = crate::BitReader<bool>;
#[doc = "Field `OFFSETPOS` writer - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OFFSETPOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR1_SPEC, bool, O>;
#[doc = "Field `SATEN` reader - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SATEN_R = crate::BitReader<bool>;
#[doc = "Field `SATEN` writer - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR1_SPEC, bool, O>;
#[doc = "Field `OFFSET_CH` reader - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\\[11:0\\]
applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers."]
pub type OFFSET_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET_CH` writer - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\\[11:0\\]
applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers."]
pub type OFFSET_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFFSET_EN` reader - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\\[11:0\\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OFFSET_EN_R = crate::BitReader<bool>;
#[doc = "Field `OFFSET_EN` writer - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\\[11:0\\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OFFSET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - Data offset y for the channel programmed into bits OFFSET_CH\\[4:0\\]
These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\\[4:0\\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\\[4:0\\]
= 4 and OFFSET2_CH\\[4:0\\]
= 4, this is OFFSET1\\[11:0\\]
which is subtracted when converting channel 4."]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\\[11:0\\]
applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers."]
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\\[11:0\\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn offset_en(&self) -> OFFSET_EN_R {
        OFFSET_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset y for the channel programmed into bits OFFSET_CH\\[4:0\\]
These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\\[4:0\\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\\[4:0\\]
= 4 and OFFSET2_CH\\[4:0\\]
= 4, this is OFFSET1\\[11:0\\]
which is subtracted when converting channel 4."]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<0> {
        OFFSET_W::new(self)
    }
    #[doc = "Bit 24 - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<24> {
        OFFSETPOS_W::new(self)
    }
    #[doc = "Bit 25 - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn saten(&mut self) -> SATEN_W<25> {
        SATEN_W::new(self)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\\[11:0\\]
applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers."]
    #[inline(always)]
    #[must_use]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<26> {
        OFFSET_CH_W::new(self)
    }
    #[doc = "Bit 31 - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\\[11:0\\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn offset_en(&mut self) -> OFFSET_EN_W<31> {
        OFFSET_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC offset 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr1](index.html) module"]
pub struct OFR1_SPEC;
impl crate::RegisterSpec for OFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr1::R](R) reader structure"]
impl crate::Readable for OFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr1::W](W) writer structure"]
impl crate::Writable for OFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFR1 to value 0"]
impl crate::Resettable for OFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
